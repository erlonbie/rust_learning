struct Logger;

impl Logger {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

struct Singleton(Option<Logger>);

impl Singleton {
    fn get_instance() -> &'static mut Singleton {
        static mut SINGLETON: Singleton = Singleton(None);
        unsafe { &mut SINGLETON }
    }

    fn get_logger(&mut self) -> &mut Logger {
        if self.0.is_none() {
            self.0 = Some(Logger);
        }
        self.0.as_mut().unwrap()
    }
}

fn take_loger(l: &Logger, msg: &str) {
    l.log(msg);
}

fn take_loger2(s: &mut Singleton, msg: &str) {
    let log = s.get_logger();
    log.log(msg);
}

fn main() {
    let singleton = Singleton::get_instance();
    let logger = singleton.get_logger();
    logger.log("This is a log message");
    logger.log("This is another message");

    take_loger(logger, "Inside take_loger function");
    take_loger2(singleton, "Passing a singleton");

    take_loger(singleton.get_logger(), "Inside take_loger function");
    take_loger2(singleton, "Passing a singleton");


}

