use crate::app::opts::Opts;
use crate::app::userevents::UserEvents;

pub struct Playground {}

impl Playground {
    pub fn run(opts: &Opts) {

        // let event_loop = EventLoop::<UserEvents>::with_user_event().build();
        // {
        //     let watch_path = opts.wgsl_file.clone();
        //     std::thread::spawn(move || Self::listen(watch_path, proxy));
        // }

        // let window = Self::create_window(&event_loop);
    }
}
