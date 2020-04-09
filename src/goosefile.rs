/// @TODO:
///  - compile the goosefile as a dynamic binary included at run-time
///  - provide tools for goose to compile goosefiles
///  - ultimately load-tests are shipped with two compiled binaries:
///      o the main goose binary (pre-compiled)
///      o the goosefile dynamic binary (compiled with a goose helper)

use crate::goose::{GooseTaskSets, GooseTaskSet, GooseTaskSetState, GooseTask};

impl GooseTaskSets {
    // @TODO: auto-write this function with metaprogramming helpers
    pub fn initialize_goosefile(&mut self) {
        trace!("initialize_goosefile");

        // Register a website task set and contained tasks
        let mut website_tasks = GooseTaskSet::new("WebsiteTasks").set_weight(10);
        website_tasks.register_task(GooseTask::new("/index.html").set_weight(6).set_function(GooseTaskSetState::website_task_index));
        website_tasks.register_task(GooseTask::new("/story.html").set_weight(9).set_function(GooseTaskSetState::website_task_story));
        website_tasks.register_task(GooseTask::new("/about.html").set_weight(3).set_function(GooseTaskSetState::website_task_about));
        self.register_taskset(website_tasks);

        /*
        // Register an API task set and contained tasks
        let mut api_tasks = GooseTaskSet::new("APITasks").set_weight(3);
        //api_tasks.register_task(GooseTask::new("on_start"));
        api_tasks.register_task(GooseTask::new("listing1").set_weight(3));
        api_tasks.register_task(GooseTask::new("listing2").set_weight(3));
        api_tasks.register_task(GooseTask::new("listing3").set_weight(0));
        self.register_taskset(api_tasks);

        let empty_tasks = GooseTaskSet::new("EmptyTasks").set_weight(1);
        self.register_taskset(empty_tasks);
        */
    }
}

impl GooseTaskSetState {
    fn website_task_index(&mut self) {
        let _response = self.get("http://apache.fosciana/");
    }

    fn website_task_story(&mut self) {
        let _response = self.get("http://apache.fosciana/story.html");
    }

    fn website_task_about(&mut self) {
        let _response = self.get("http://apache.fosciana/about.html");
    }
}

/*
class WebsiteUser(HttpLocust):
    task_set = WebsiteTasks
    wait_time = between(5, 15)
*/