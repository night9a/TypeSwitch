# TypeSwitch: Hackathon Project Overview

TypeSwitch is a web-based platform designed to make file type conversion fast, easy, and extensible. Built with Rust and the Rocket web framework, it provides a simple interface for users to upload files and convert them between different formats. The project is architected for future expansion, including developer APIs and AI-powered features.

---

## What Does TypeSwitch Do?
- **File Conversion:** Instantly convert files from one type to another (e.g., images, documents, etc.).
- **Web Platform:** Clean, user-friendly web interface using Tera templates.
- **API-Ready:** The backend is designed to expose RESTful APIs for integration with other apps and services.
- **Future-Proof:** Modular codebase for adding authentication, persistent storage, and AI features.

---

## Technical Highlights
- **Language:** Rust
- **Framework:** Rocket (with Tera templates)
- **Multipart Uploads:** Handles file uploads via web forms and APIs.
- **Configurable:** Uses `Rocket.toml` for easy environment setup.

---

## Architecture (Current & Planned)
```
src/
├── main.rs        # Rocket setup and route mounting
├── core/          # (Planned) Business logic and services
├── web/           # (Planned) Web page handlers
├── api/           # (Planned) API endpoints
├── db/            # (Planned) Database models and access
templates/         # Tera HTML templates
Rocket.toml        # Rocket configuration
```

---

## Future Vision
- **Authentication:** User accounts and file history.
- **Persistent Storage:** Save and manage converted files.
- **Developer API:** Public endpoints for third-party integration.
- **AI Features:** Smart file analysis, recommendations, and advanced conversions.

---

*This project was built for a hackathon to showcase rapid development, clean architecture, and extensibility using Rust.* 