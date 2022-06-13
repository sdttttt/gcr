#![cfg(feature = "plug")]

use git2::Error;
use std::rc::Rc;

use crate::repo::Repository;

mod log;
mod push;

use log::LogPlugin;
use push::PushPlugin;

pub trait CommitPlugin {
	fn before(&self, _: &Repository) -> Result<(), Error> {
		Ok(())
	}

	fn after(&self, _: &Repository) -> Result<(), Error> {
		Ok(())
	}
}

pub fn find_plug(plug_names: &Vec<String>) -> Vec<Rc<dyn CommitPlugin>> {
	let plugin_all: &[(&str, Rc<dyn CommitPlugin>); 2] =
		&[("log", Rc::new(LogPlugin::new())), ("push", Rc::new(PushPlugin::new()))];

	plugin_all
		.iter()
		.filter(|t| plug_names.contains(&t.0.to_owned()))
		.map(|t| t.1.clone())
		.collect::<Vec<Rc<dyn CommitPlugin>>>()
}
