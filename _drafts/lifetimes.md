---
layout: post
title: lifetime variables and safety
---

17 November 2013

Like its C++ counterpart,
capnproto-rust relies heavily
on pointer manipulation
to achieve performance.
In fact,
the translation
from C++ to Rust
for the core low level
data structures
is quite direct,
as you can see by comparing
layout.c++ and layout.rs.

One important  difference is that
Rust, in addition to
requiring that we explicitly
label the pointer manipulations as "unsafe",
also provides us
with better facilities
for building
a safe external interface
on top of them.

To illustrate, let's look at some
ways to misuse the C++ interface.

Here's a program that builds two
`Person` structs, each a part of
its own `AddressBook` message.

```
#include "addressbook.capnp.h"
#include <capnp/message.h>
#include <iostream>

using addressbook::Person;
using addressbook::AddressBook;

Person::Builder returnPersonBuilder(int id) {
  ::capnp::MallocMessageBuilder message;

  auto addressBook = message.initRoot<AddressBook>();
  auto people = addressBook.initPeople(1);

  people[0].setId(id);
  people[0].setName("Alice");

  return people[0];
}

int main(int argc, char* argv[]) {
  auto person1 = returnPersonBuilder(123);
  auto person2 = returnPersonBuilder(456);
  std::cout << person1.getId() << "\n";
  return 0;
}

```

You might expect the program to print
"123", but it actually prints "456".
The `Person::Builder` returned
by the `returnPersonBuilder()` function
is unsafe to use because it
outlives its `MessageBuilder`.

Here is a snippet showing a related problem.

```
{
  ::capnp::MallocMessageBuilder message;

  auto addressBook = message.initRoot<AddressBook>();
  auto people = addressBook.initPeople(1);

  auto alice = people[0];
  alice.setId(123);

  auto person = message.initRoot<Person>();

  std::cout << alice.getId() << "\n";
}
```
You might expect this code to print "123", but
it actually prints "0" because `alice`
is no longer valid after `message` has
been initialized a second time.

Both of these errors could be statically
detected and prevented in Rust.
The key is to arrange that the
`MessageBuilder::initRoot()` function
*borrow* a reference to the message that invokes it,
and to keep track of the *lifetime* of that borrow.
The Rust typechecker will then be able to detect
if the message is borrowed again
or if some sub-builder of it---whose type will
be annotated with the lifetime---.

To make this concrete...


