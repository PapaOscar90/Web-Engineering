# Backend Installation Instructions

Our backend is powered by Rust and Rocket. Check it our on their homepage: https://www.rust-lang.org/.

## Steps:
Install Rust
```sh
curl https://sh.rustup.rs -sSf | sh  # Install Rustup
rustup default nightly               # Install nightly version of the compiler
cd backend                           # Navigate to the backend project directory
```

Install CockroachDB
```sh
wget -qO- https://binaries.cockroachdb.com/cockroach-v2.1.6.linux-amd64.tgz | tar  xvz
cp -i cockroach-v2.1.6.linux-amd64/cockroach /usr/local/bin
```

Install diesel
```sh
cargo install diesel_cli --no-default-features --features postgres
```

Run in new terminal
```sh
cockroach start --insecure
```

Now
```sh
diesel database setup              # Setup the database
cargo run --release                # Run the project
                                   # Omit --release for dev version (faster to compile)
```

# Frontend Installation Instructions

Our frontend is powered by ReasonML and Reason-React. Check it out on their GitHub page: https://reasonml.github.io/

## Steps:
- curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.34.0/install.sh | bash
- nvm install node
- npm install --global yarn
  - // Make sure yarn is on your PATH
- yarn global add bs-platform
- npx create-react-app frontend --scripts-version reason-scripts
- yarn start
  - // Now install your editor plugin (reasonml.github.io/docs/en/editor-plugins)
  - // Saving will auto-refresh the page for you for 'live-preview'
