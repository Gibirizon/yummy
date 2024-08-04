# Yummy 🍽️

Yummy is a modern web application designed for finding and sharing recipes. It leverages the power of the Internet Computer (IC), providing a decentralized and scalable solution.

## Screenshots

## Features

- **Recipe Sharing:** Discover and share a variety of recipes with the community.
- **User-Friendly UI:** Intuitive, responsive, and easy-to-use interface for a seamless user experience.
- **Internet Identity Login:** Securely log in using your Internet Identity, providing a seamless and secure authentication experience.
- **Internet Computer Integration:** Utilizes the Internet Computer network for a scalable, decentralized infrastructure.

## Technologies

Built with:

- [Rust 🦀](https://github.com/rust-lang/rust) for backend
- [Vue.js](https://github.com/vuejs/core) for frontend
- [Pinia](https://github.com/vuejs/pinia) 🍍 for state management
- [TailwindCSS](https://github.com/tailwindlabs/tailwindcss) 🍃 for styling
- [DfinityCDK](https://github.com/dfinity/cdk-rs) for compiling to the Internet Computer

## Getting Started

### Prerequisites

- Node.js (>= 16)
- npm (>= 7)
- [dfx](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) (DFINITY Canister SDK)

### Running the Project Locally

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/Gibirizon/yummy.git
   cd yummy
   ```

2. **Install Dependencies:**

   ```sh
   npm install
   ```

3. **Start the Internet Computer Replica:**

   ```sh
   dfx start --background
   ```

4. **Deploy the Canisters:**

   ```sh
   dfx deploy
   ```

5. **Run the Development Server:**

   ```sh
   npm start
   ```

   Access the application at `http://localhost:3000`. The development server proxies API requests to the IC replica running on port 4943.

Enjoy discovering and sharing recipes on Yummy!🍳👨‍🍳👩‍🍳
