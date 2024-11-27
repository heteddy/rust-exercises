use std::io;
use tokio::time::{self, Duration, Instant};
use tokio::{self, sync::watch};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}
impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
        println!("loading from file {:?}", Instant::now());
        Ok(Config {
            timeout: Duration::from_millis(1000),
        })
    }
}
async fn my_async_operation() {
    // Do something here
    println!("my_async_operation start {:?}", Instant::now());
    tokio::time::sleep(Duration::from_millis(1500)).await;
    println!("my_async_operation end {:?}", Instant::now());
}

mod tests {
    use super::*;
    #[test]
    fn test_watch_sleep() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Load initial configuration value
            let mut config = Config::load_from_file().await.unwrap();
            // Create the watch channel, initialized with the loaded configuration
            let (tx, rx) = watch::channel(config.clone());
            // Spawn a task to monitor the file.
            let loop_ret = tokio::spawn(async move {
                loop {
                    // Wait 10 seconds between checks
                    // time::sleep(Duration::from_secs(10)).await; // Load the configuration file
                    let new_config = Config::load_from_file().await.unwrap();
                    // If the configuration changed, send the new config value // on the watch channel.
                    if new_config != config {
                        tx.send(new_config.clone()).unwrap();
                        config = new_config;
                    }
                    let mut handles = vec![];
                    // Spawn tasks that runs the async operation for at most `timeout`. If // the timeout elapses, restart the operation.
                    //
                    // The task simultaneously watches the `Config` for changes. When the // timeout duration changes, the timeout is updated without restarting // the in-flight operation.

                    for _ in 0..5 {
                        // Clone a config watch handle for use in this task
                        let mut rx = rx.clone();
                        let handle = tokio::spawn(async move {
                            // Start the initial operation and pin the future to the stack. // Pinning to the stack is required to resume the operation
                            // across multiple calls to `select!`
                            let op = my_async_operation();
                            tokio::pin!(op);
                            // Get the initial config value
                            let mut conf = rx.borrow().clone(); // 取出初始值
                            let op_start = Instant::now();
                            let sleeping = time::sleep_until(op_start + conf.timeout);
                            tokio::pin!(sleeping);
                            loop {
                                tokio::select! {
                                    _ = &mut sleeping => {
                                            println!("sleeping 1... :{:?}",Instant::now());                // The operation elapsed. Restart it
                                             // Track the new start time
                                            // op_start = Instant::now();
                                            // // Restart the timeout
                                            // sleep.set(time::sleep_until(op_start + conf.timeout));
                                        }
                                    _ = rx.changed() => {
                                        println!("changed 2...{:?}",Instant::now());
                                        conf = rx.borrow_and_update().clone();
                                        println!("config updated = {:?},{:?}",conf,Instant::now());
                                        // The configuration has been updated. Update the // `sleep` using the new `timeout` value. sleep.as_mut().reset(op_start + conf.timeout);
                                    }
                                    _ = &mut op => {
                                        println!("op 3...{:?}",Instant::now());
                                        op.set(my_async_operation());
                                        // The operation completed!
                                        return
                                    }
                                }
                            }
                        });
                        handles.push(handle);
                    }
                    for handle in handles.drain(..) {
                        println!("await result = {:?}", handle.await.unwrap());
                    }
                }
            });
            loop_ret.await.unwrap();
        });
    }
}
