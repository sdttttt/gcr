use std::fmt::Error;
use std::rc::Rc;

use crate::repo::Repository;

mod log;
mod push;

use log::LogPlugin;
use push::PushPlugin;

pub trait CommitPlugin {
	fn before(&self, repo: &Repository, msg: &String) -> Option<Error>;
	fn after(&self, repo: &Repository, msg: &String) -> Option<Error>;
}

pub fn find_plug(plug_names: &Vec<String>) -> Vec<Rc<dyn CommitPlugin>> {
	let plugin_all: &[(&str, Rc<dyn CommitPlugin>); 2] =
		&[("log", Rc::new(LogPlugin::new())), ("push", Rc::new(PushPlugin::new()))];

	let mut plugs = vec![];
	for name in plug_names {
		for plug in plugin_all {
			if name.as_str() == plug.0 {
				plugs.push(Rc::clone(&plug.1));
			}
		}
	}

	plugs
}
