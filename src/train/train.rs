use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, TryRecvError};
use std::time::Duration;

use atomicwrites::{AtomicFile, Error as AtomicError, OverwriteBehavior};
use bincode::{deserialize_from, serialize_into};
use failure::Error;
use futures::{Async, stream::poll_fn};
use inflector::numbers::ordinalize::ordinalize;
use neuroflap_neat::Population;
use neuroflap_world::{run_one, Event};
use rand::XorShiftRng;

use util::SharedRng;

/// Options taken by the `train` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// The generation being trained.
    pub generation_file: PathBuf,

    /// A directory where generation snapshots are stored. If not present,
    /// generation snapshots will not be made.
    #[structopt(long = "results")]
    pub results_dir: Option<PathBuf>,
}

impl Options {
    /// Runs for several generations.
    pub fn run(self) -> Result<(), Error> {
        let mut pop: Population = {
            let f = File::open(&self.generation_file)?;
            deserialize_from(f)?
        };

        let mut rng = SharedRng::new(XorShiftRng::new_unseeded());

        loop {
            info!("Training generation {}...", pop.generation());
            pop = pop.run_generation(&mut rng.clone(), |genome| {
                let network = genome.build_network(pop.params.activation);
                let (send, recv) = channel();
                run_one(
                    poll_fn(|| -> Result<_, !> {
                        match recv.try_recv() {
                            Ok(x) => Ok(Async::Ready(Some(x))),
                            Err(TryRecvError::Empty) => Ok(Async::NotReady),
                            Err(TryRecvError::Disconnected) => {
                                Ok(Async::Ready(None))
                            }
                        }
                    }),
                    |world| {
                        let (next_pipe_x, next_pipe_y) = world
                            .pipes
                            .iter()
                            .cloned()
                            .filter(|&(x, _)| x >= 0.5)
                            .next()
                            .unwrap_or((0.0, 0.0));
                        debug!("{} {}", next_pipe_x, next_pipe_y);

                        let out = network.calculate([
                            world.position,
                            next_pipe_x,
                            next_pipe_y,
                            world.velocity,
                        ]);

                        if out > 0.5 {
                            send.send(Event::Jump)?;
                        }
                        Ok(())
                    },
                    &mut rng,
                    || Duration::from_millis(50),
                ).map(|s| s.unwrap())
            })?;

            info!(
                "Finished training {} generation",
                ordinalize(&format!("{}", pop.generation()))
            );

            if let Some(results_dir) = self.results_dir.as_ref() {
                let file = format!("{}.gen", pop.generation());
                let mut file = File::create(results_dir.join(file))?;
                serialize_into(file, &pop)?;
            }
            update_generation_file(&self.generation_file, &pop)?;
        }
    }
}

fn update_generation_file<P: AsRef<Path>>(
    path: P,
    pop: &Population,
) -> Result<(), Error> {
    AtomicFile::new(path, OverwriteBehavior::AllowOverwrite)
        .write(|file| serialize_into(file, pop).map_err(Error::from))
        .map_err(|err| match err {
            AtomicError::Internal(err) => err.into(),
            AtomicError::User(err) => err,
        })
}
