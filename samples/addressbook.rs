#[link(name = "capnproto-rust-test", vers = "alpha", author = "dwrensha")];

#[crate_type = "bin"];

extern mod capnprust;

use capnprust::*;

pub mod addressbook_capnp;
pub mod schema_capnp;

fn addressbook() {
    use capnprust::message::*;
    use addressbook_capnp::*;

    let message = MessageBuilder::new_default();

    let addressbook = AddressBook::Builder::new(message.initRoot(AddressBook::STRUCT_SIZE));

    let people = addressbook.initPeople(4);

    let person = people.get(0);
    person.setId(1);
    person.setName("Alice");
    person.setEmail("alice@widgco.biz");
    let phones = person.initPhones(2);
    phones.get(0).setNumber("(555) 555-5555");
    phones.get(0).setType(Person::PhoneNumber::Type::WORK);
    phones.get(1).setNumber("(777) 123-4567");
    phones.get(1).setType(Person::PhoneNumber::Type::HOME);
    person.getEmployment().setEmployer("widgco");

    let person = people.get(1);
    person.setId(2);
    person.setName("Bob");
    person.getEmployment().setSelfEmployed(());

    let person = people.get(2);
    person.setId(3);
    person.setName("Charlie");
    person.getEmployment().setUnemployed(());

    let person = people.get(3);
    person.setId(255);
    person.setName("Diane");
    person.getEmployment().setSchool("Caltech");

    let outStream = @std::io::stdout() as @serialize::OutputStream ;

//    serialize::writeMessage(outStream, message)
    serialize_packed::writePackedMessage(outStream, message)
}

fn schema() {
    use capnprust::message::*;

    use schema_capnp::*;

    let message = MessageBuilder::new_default();

    let request = CodeGeneratorRequest::Builder::new(
        message.initRoot(CodeGeneratorRequest::STRUCT_SIZE));

    request.initNodes(1);

    let outStream = @std::io::stdout() as @serialize::OutputStream;
//    serialize_packed::writePackedMessage(outStream, message)
    serialize::writeMessage(outStream, message)
}


fn main() {

    let args = std::os::args();
    if (args.len() < 2) {
        std::io::println(fmt!("usage: $ %s [addressbook | schema]", args[0]));
    } else {
        match args[1] {
            ~"addressbook" => addressbook(),
            ~"schema" => schema(),
            _ => {std::io::println("unrecognized argument") }
        }
    }

}