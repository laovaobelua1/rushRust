use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PeoplePlugin)
    .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
           .add_system(print_names)
           .add_system(people_with_jobs)
           .add_system(people_ready_for_hire)
           .add_system(print_name_and_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
        name: "Alice".to_string(),
    },
        Employed {
        job: Job::Engineer,
    }
    ));
    commands.spawn((Person {
        name: "Bob".to_string(),
    },
        Employed {
        job: Job::Artist,
    }
    ));
    commands.spawn((Person {
        name: "Alan".to_string(),
    },
        Employed {
        job: Job::Teacher,
    }
    ));
    commands.spawn((Person {
        name: "Method".to_string(),
    },
        Employed {
        job: Job::Engineer,
    }
    ));
    commands.spawn(Person {
        name: "Alexander".to_string(),
    });
    commands.spawn((Person {
        name: "Huy dep trai".to_string(),
    },
        Employed {
        job: Job::random(),
    }
    ));
    commands.spawn((Person {
        name: "Alan".to_string(),
    },
        Employed {
        job: Job::random(),
    }
    ));
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Person name: {}", person.name);
    }
}

pub fn print_name_and_job(
    person_query: Query<(&Person, Option<&Employed>)>
) {
    for (person, employed) in person_query.iter() {
        match employed {
            Some(employed) => {
                println!("Person name: {}, Job: {:?}", person.name, employed.job);
            }
            None => {
                println!("Person name: {}, Job: Unemployed", person.name);
            }
        }
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>
) {
    for person in person_query.iter() {
        println!("Employed person name: {}", person.name);
    }
}

pub fn people_ready_for_hire(
    person_query: Query<&Person, Without<Employed>>
) {
    for person in person_query.iter() {
        println!("Person ready for hire: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}
#[derive(Component)]
pub struct Employed {
    pub job: Job, 
}

#[derive(Debug)]

pub enum Job {
    Engineer,
    Artist,
    Teacher,
}

impl Job {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Job::Engineer,
            1 => Job::Artist,
            2 => Job::Teacher,
            _ => unreachable!(),
        }
    }
}
//commit