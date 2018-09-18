# tbd.rs

`tbd.rs` (Name: To Be Determined) is a draft for a database mapper in Rust.

It takes inspiration from [sequel](https://sequel.jeremyevans.net/), [rom.rb](https://rom-rb.org) and [ecto](https://hexdocs.pm/ecto/Ecto.html).

This document serves as a write-up of the current state of work and the thinking behind it.

## Goals

`tbd.rs` has a couple of design goals:

* Modern, Rust 2018 code
    * futures 0.3 (async/await) compatibility
* async first
    * scalars are returned as futures
    * collections are returned as streams
        * potentially infinite, allowing connection to queues
* No ORM
    * tbd models have no knowledge of their persistence
    * mapping from the results of a database query to the
      domain models is a separate, explicit step
        *  standard implementations can be generated
* No attempt to hide the database
    * Database systems should be used for the features they have, not the ones they share with others
    * Common features should be shared
    * Database specific features should be accessible on top

## Current implementation description

### Notes

Many of the traits below are intended to be implemented on zero-sized types. This means the abstraction vanishes at runtime and takes up no memory.


### `Repository`

A `Repository` is an abstract store of data. It has no direct representation in the database, but serves as a model to describe the contents of one.

Currently, a `Repository` is tied to a `Gateway` (though that relationsihp might be generic). The Repository is the primary interface to the data store.

A `Repository` defines the scope of queries, as cross-repository queries are currently not allowed.

Example:

```rust
struct BlogRepository {
    gateway: Sqlite3Gateway
}

impl Repository for BlogRepository {
    type Gateway = Sqlite3Gateway;

    fn gateway(&self) -> &Sqlite3Gateway {
        &self.gateway
    }
}

async fn write_to(repos: &BlogRepository) {
    let mut changeset = BlogRepository::change().inserts::<Posts>();

    for id in 1..=3 {
        let post = Post { content: format!("Post number {}", id) };

        changeset.push(post);
    }

    changeset.commit(&repos);
}

async fn read_from(repos: &BlogRepository) {
    let query = select::<Post>().from::<Posts>();

    let e1 = query.execute(&repos).for_each(|item| {
        println!("{:?}", item);
        future::ready(())
    });
}
```

### `Relation`

`Relation` describes a single database relation, for example a database table. A relation has a primary key and the items stored within. It is _independent_ of a `Repository`. It is perfectly fine to store Relations in multiple Repositories.

A `Relation` should also hold important information such as its name or the names of fields contained.

It _currently_ holds information on how to serialise a model, but that's to be removed.

Currently, `Relation` holds a `Wrapper` type, which allows wrapping and unwrapping of models into wrapping types, providing e.g. primary keys and `updated_at`, `created_at` timestamps are present.

Relations are the anchors queries operate over.

```rust
struct Posts;

impl Relation for Posts {
    type PrimaryKey = i64;
    type Model = Post;
    type Wrapper = KeyedPost;

    fn hydrate(model: &KeyedPost) -> HashMap<String, String> {
        let model = &model.0;
        let mut h = HashMap::new();
        if let Some(id) = model.pk {
            h.insert("id".to_string(), id.to_string());
        }
        h.insert("content".to_string(), format!("{}{}{}", '"', model.content.to_string(), '"'));
        h
    }

    fn name() -> &'static str {
        "posts"
    }
}
```

### `Stores<T>`

The `Stores` trait describes that a `Relation` is stored within a `Repository`. It allows reasoning about the location of `Relation`s. For example, `Posts` and `Comments` are in the same `Repository` if the `Repository` fulfills the bound `Stores<Posts> + Stores<Comments>`.

Note that this also means that a `Relation` can only be stored in a `Repository` once. This is intentional.

Any pair of `Relation`s stored is also considered stored, e.g. if a `Repository` is `Stores<Posts> + Stores<Comments>`, it is also `Stores<Posts, Comments>`

### `Relationship`

`Relationship`s are currently in an early draft phase. They are a set of traits that describe different ways of seeing `Relation`s together. This allows the expression of queries later.

```rust
struct PostComments;

impl HasManyRelationShip for PostComments {
    type Of = Posts;
    type To = Comments;
}
```

### `Gateway`

The `Gateway` finally implements all concrete interaction with the storage. It maps all input directly to a database driver or storage client.

### `Query`

A query describes reading out of relations. It uses the information stored about relations and repositories, but can ultimately be specific to a `Gateway`. As an example, SQL queries are mostly generic, but some features only work on Postgres. In this case, it is perfectly feasible to use a query implementation tailored for Postgres. Currently, only a mock implementation is provided.

Gateways are responsible for compiling the queries and then sending them to the storage.

### `Changeset`

Changesets are used for storing data. A `Changeset` can only span one `Repository`, it can change multiple `Relation`s, though. The `Gateway` is then responsible for executing the changeset. `Changeset`s are constructed through a `Repository`, which they can then be applied to.

```

struct BlogRepository {
    gateway: Sqlite3Gateway
}

impl Repository for BlogRepository {
    type Gateway = Sqlite3Gateway;

    fn gateway(&self) -> &Sqlite3Gateway {
        &self.gateway
    }
}

async fn write_to(repos: &BlogRepository) {
    let mut changeset = BlogRepository::change().inserts::<Posts>();

    for id in 1..=3 {
        let post = Post { content: format!("Post number {}", id) };

        changeset.push(post);
    }

    changeset.commit(&repos);
}
```

### The missing `Model`

`tbd.rs` tries to avoid relying on a `Model` type; this is a code smell outside of ORMs. `tbd.rs` maps database queries to domain models and completely describes that mapping step.

### Why concrete types?

As you can see , `tbd.rs` often uses concrete types with traits implemented on top to model e.g. a `Relation` `Posts` storing `Post`-models will have a concrete type `Posts`, even if it is zero-sized. This has multiple advantages:

* It makes type resolution easier
* It allows new types with similar implementations to be introduced (such as a `Relation` `Drafts`, also storing `Post`)
* In the case of relationships, it allows multiple relationships of similar form, as the types won't collide, a frequent issue if those were a trait on the relationship
* Those types are addressable everywhere

Zero-Sized types introduce names and labels into the type system, `tbd.rs` uses them effectively to express relationships between those and uses them at compile time to generate queries.

This also allows for effective code generation later.
