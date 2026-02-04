use ixa::prelude::*;
use ixa_vaccines::VaccineEfficacy;

define_entity!(Person);

define_property!(struct Age(u8), Person);

define_property!(
    enum InfectionStatus {
        S,
        I,
        R,
    },
    Person
);

impl_property!(VaccineEfficacy, Person);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    let person = context.add_entity::<Person, _>((InfectionStatus::S,))?;
    context.set_property::<Person, _>(person, VaccineEfficacy(1.0));
    assert_eq!(
        context.get_property::<Person, InfectionStatus>(person),
        InfectionStatus::S
    );
    Ok(())
}
