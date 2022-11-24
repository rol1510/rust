// aux-build:inline-default-methods.rs
// ignore-cross-compile

extern crate inline_default_methods;

// @has inline_default_methods/trait.Foo.html
// @has - '//*[@class="rust trait"]' '# Required method fn bar(&self);'
// @has - '//*[@class="rust trait"]' '# Provided method fn foo(&mut self);'
pub use inline_default_methods::Foo;

// @has inline_default_methods/trait.Bar.html
// @has - '//*[@class="rust trait"]' '# Required method fn bar(&self);'
// @has - '//*[@class="rust trait"]' '# Provided methods fn foo1(&mut self);'
// @has - '//*[@class="rust trait"]' 'fn foo2(&mut self);'
pub use inline_default_methods::Bar;

// @has inline_default_methods/trait.Baz.html
// @has - '//*[@class="rust trait"]' '# Required methods fn bar1(&self);'
// @has - '//*[@class="rust trait"]' 'fn bar2(&self);'
// @has - '//*[@class="rust trait"]' '# Provided method fn foo(&mut self);'
pub use inline_default_methods::Baz;
