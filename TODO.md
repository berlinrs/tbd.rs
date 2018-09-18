* Changeset should start from repository: `repos.users.change()`
    * Relation should create items `Users.create()`
    * Relation may wrap user: `Id<Timestamps<User>>`
* Relations should hold
    * `fields() -> Fieldset`: a list of all fields
    * Fields under their names (e.g. `Users.name() -> impl Field<User>`)
    * Retrieve fields by type
      * `Users::field::<T: Field>() -> impl Field<User>` 
    * `select::<(user.id, user.name)>` should be possible
* Query should start from repository `repos.users.all()`
* There should be a second insertion interface that works without changesets and allows you to
  stream into a database. No guarantee for success, though.
* Wrapper Trait needs to be reworked

* Fields API
    * An API to retrieve the field list of a relation
