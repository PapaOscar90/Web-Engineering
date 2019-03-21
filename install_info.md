# Backend Installation Instructions

Our backend is powered by Rust and Rocket. Check it our on their homepage: https://www.rust-lang.org/.

## Steps:
```sh
curl https://sh.rustup.rs -sSf | sh # Install Rustup
rustup default nightly              # Install nightly version of the compiler
cd backend                          # Navigate to the backend project directory
cargo run                           # Run the project
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
