# There be Dinos

This is a basic mock for DinoParks dependency on [CIS](https://github.com/mozilla-iam/cis/).
It utilizes the output of [iam-profile-faker](https://github.com/mozilla-iam/iam-profile-faker)
and provides the following end-points:

- `/cisUpdate` receives (partial) profile updates and returns an `update_id`
- `/cisStatus/` provides the status of a profile update and returns an `user_id` on success
- `/personApi/` allows to get a full profile for a given `user_id`
- `/admin/users` lists all `user_id`s in the vault
- `/admin/persist` persist all pending update to the database (file)

## Development

### Install Rust

Install `rustup`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install the nightly toolchain:

```
rustup install nightly
```

### Install the Profile Faker

```
pip install iam_profile_faker
```

### Run DinoCave

Set your profile store location:
```
export DC_PROFILE_STORE=/some/path/file.json
```

First generate some profiles:

```
iam_profile_faker create_batch --count 100 > $DC_PROFILE_STORE
```

```
cargo +nightly run
```