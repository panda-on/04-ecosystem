use anyhow::Result;
use chrono::{DateTime, Datelike, Utc};
use derive_builder::Builder;

// use Builder derive macro to implement builder pattern for User struct
#[allow(unused)]
#[derive(Builder, Debug)]
#[builder(build_fn(name = "_priv_build"))]
struct User {
    #[builder(setter(into))]
    name: String,

    #[builder(setter(into, strip_option))]
    email: Option<String>,

    #[builder(setter(custom))]
    dob: DateTime<Utc>,

    #[builder(setter(skip))]
    age: u32,

    #[builder(default = "vec![]", setter(each(name = "skill", into)))]
    skills: Vec<String>,
}

fn main() -> Result<()> {
    let user3 = User::build()
        .name("John Doe")
        .skill("programming")
        .skill("piano")
        .skill("AI")
        .email("Doe@nice.com")
        .dob("2000-01-01T00:00:00Z")
        .build()?;

    println!("{:?}", user3);
    Ok(())
}

impl User {
    fn build() -> UserBuilder {
        UserBuilder::default()
    }
}

impl UserBuilder {
    pub fn dob(&mut self, dob: &str) -> &mut Self {
        self.dob = DateTime::parse_from_rfc3339(dob)
            .map(|dt| dt.with_timezone(&Utc))
            .ok();

        self
    }

    pub fn build(&self) -> Result<User> {
        let mut user = self._priv_build()?;
        user.age = (Utc::now().year() - user.dob.year()) as u32;
        Ok(user)
    }
}
