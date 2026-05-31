mod common;

use common::message::Role;

fn main() {
    println!("Forge: An Agentic AI Harness");
    let role: Role = Role::User;
    println!("Role: {:#?}", role);
}
