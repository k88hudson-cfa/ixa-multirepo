use ixa::prelude::*;
use ixa_vaccines::{Person, VaccineEfficacy};

// | Entity   | Property | Macro            | Example                              |
// |----------|----------|------------------|--------------------------------------|
// | Local    | -        | define_entity!   | Compartment                          |
// | Foreign  | -        | -                | Person (from ixa_vaccines)           |
// | Local    | Local    | define_property! | Age on Compartment                   |
// | Local    | Local    | define_property! | InfectionStatus on Compartment       |
// | Foreign  | Local    | impl_property!   | InfectionStatus on Person            |
// | Local    | Foreign  | impl_property!   | VaccineEfficacy on Compartment       |

define_entity!(Compartment);
define_property!(struct Age(u8), Compartment);
define_property!(
    enum InfectionStatus {
        S,
        I,
        R,
    },
    Compartment
);
impl_property!(InfectionStatus, Person);
impl_property!(VaccineEfficacy, Compartment);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();

    // Add local entity, local prop
    let compartment = context.add_entity::<Compartment, _>((InfectionStatus::S,))?;
    // Add foreign entity, foreign prop
    let person = context.add_entity::<Person, _>((VaccineEfficacy(1.0),))?;
    // Add foreign entity, local prop
    let person = context.add_entity::<Person, _>((InfectionStatus::S,))?;

    // Set local entity, foreign prop
    context.set_property::<Compartment, _>(compartment, VaccineEfficacy(1.0));
    assert_eq!(
        context.get_property::<Compartment, InfectionStatus>(compartment),
        InfectionStatus::S
    );

    Ok(())
}
