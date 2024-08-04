# Yummy 🍽️

Yummy is a modern web application designed for finding and sharing recipes. It leverages the power of the Internet Computer (IC), providing a decentralized and scalable solution.

Built with:

- [Rust 🦀](https://github.com/rust-lang/rust) for backend
- [Vue.js](https://github.com/vuejs/core) for frontend
- [Pinia](https://github.com/vuejs/pinia) 🍍 for state management
- [TailwindCSS](https://github.com/tailwindlabs/tailwindcss) 🍃 for styling
- [DfinityCDK](https://github.com/dfinity/cdk-rs) for compiling to the Internet Computer

## Screenshots

[Add your screenshots here]

## Features

- **Recipe Sharing:** Discover and share a variety of recipes with the community.
- **User-Friendly UI:** Intuitive, responsive, and easy-to-use interface for a seamless user experience.
- **Internet Identity Login:** Securely log in using your Internet Identity, providing a seamless and secure authentication experience.
- **Internet Computer Integration:** Utilizes the Internet Computer network for a scalable, decentralized infrastructure.

## Technologies

<div style="display: flex; align-items: center;">
  <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="Rust" width="50" height="50"/>
  <span style="margin-left: 10px; font-size: 1.2em;">Rust Backend</span>
</div>
<div style="display: flex; align-items: center; margin-top: 10px;">
  <img src="https://vuejs.org/images/logo.png" alt="Vue.js" width="50" height="50"/>
  <span style="margin-left: 10px; font-size: 1.2em;">Vue.js Frontend</span>
</div>
<div style="display: flex; align-items: center; margin-top: 10px;">
  <img src="https://tailwindcss.com/_next/static/media/tailwindcss-logotype.c55cdf77c96ef34e74009c6ec2d9f13c.svg" alt="Tailwind CSS" width="150" height="50"/>
  <span style="margin-left: 10px; font-size: 1.2em;">Tailwind CSS</span>
</div>
<div style="display: flex; align-items: center; margin-top: 10px;">
  <img src="https://internetcomputer.org/img/logo.svg" alt="Internet Computer" width="50" height="50"/>
  <span style="margin-left: 10px; font-size: 1.2em;">Internet Computer</span>
</div>

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
