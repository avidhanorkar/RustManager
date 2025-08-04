# RustManager - Task Management System

Well the project "Rust Manager" is a task manager which is a pretty basic one. But I am trying to learn Rust and I want to build a project which will help me learn Rust. So I decided to make a task manager such that with the help of this project I can learn the Axum framework on the go.

## 🚀 Project Overview

RustManager is a full-stack task management application that provides secure user authentication and task management capabilities. The backend is built with Rust using the Axum framework, MongoDB for data persistence, and JWT for secure authentication.

## 🏗️ Architecture

### Backend (Rust + Axum)
- **Framework**: Axum (async web framework)
- **Database**: MongoDB with official Rust driver
- **Authentication**: JWT (JSON Web Tokens)
- **Password Hashing**: bcrypt
- **Async Runtime**: Tokio

### Key Features
- User registration and authentication
- JWT-based secure API access
- CRUD operations for tasks
- MongoDB integration with ObjectId handling
- Middleware-based authentication
- Error handling and validation

## 📁 Project Structure

```
RustManager/
└── backend/
    ├── src/
    │   ├── controller/
    │   │   ├── auth_controller.rs    
    │   │   ├── task_controller.rs    
    │   │   └── mod.rs
    │   ├── middleware/
    │   │   ├── auth_middleware.rs    
    │   │   └── mod.rs
    │   ├── models/
    │   │   ├── user_model.rs         
    │   │   ├── task_model.rs         
    │   │   └── mod.rs
    │   ├── routes/
    │   │   ├── router.rs             
    │   │   └── mod.rs
    │   ├── utils/
    │   │   ├── db.rs                 
    │   │   └── mod.rs
    │   └── main.rs                   
    ├── Cargo.toml                    
    └── .gitignore
                      
```

## 🔧 Prerequisites

- **Rust**: Latest stable version (1.70+)
- **MongoDB**: Local or cloud instance
- **Environment Variables**: Configure required environment variables

## 🛠️ Installation & Setup

### 1. Clone the Repository
```bash
git clone <https://github.com/avidhanorkar/RustManager>
cd RustManager/backend
```

### 2. Install Dependencies
```bash
cargo build
```

### 3. Environment Configuration
Create a `.env` file in the backend directory:
```bash
# Required environment variables
MONGODB_URI=mongodb://localhost:27017
JWT_SECRET=your-secret-key-here
PORT=3000
```

### 4. Database Setup
Ensure MongoDB is running:
```bash
# Start MongoDB (if using local)
mongod
```

### 5. Run the Application
```bash
# Development mode
cargo run

# Production build
cargo build --release
./target/release/backend
```

## 📡 API Endpoints

### Authentication Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/user/register` | Register new user |
| POST | `/user/login` | User login |
| GET | `/user` | Get current user data |

### Task Management Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/task/create` | Create new task |
| PATCH | `/task/update/{task_id}` | Update existing task |
| GET | `/task/getAll` | Get all tasks for current user |

### Health Check
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Basic health check |

## 🔐 Authentication

The API uses JWT tokens for authentication. After successful login, include the token in the Authorization header:

```bash
Authorization: Bearer <your-jwt-token>
```

## 📊 Data Models

### User Model
```rust
User {
    user_id: Option<ObjectId>,
    username: String,
    email: String,
    password: String, // Hashed
    tasks: Vec<ObjectId>, // References to Task documents
}
```

### Task Model
```rust
Task {
    task_id: Option<ObjectId>,
    taskname: String,
    status: String, // "Pending", "In Progress", "Completed"
    user_id: String, // Owner reference
}
```

## 🧪 API Usage Examples

### Register New User
```bash
curl -X POST http://localhost:3000/user/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepassword"
  }'
```

### User Login
```bash
curl -X POST http://localhost:3000/user/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "securepassword"
  }'
```

### Create Task (with authentication)
```bash
curl -X POST http://localhost:3000/task/create \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -d '{
    "taskname": "Complete project documentation",
    "status": "In Progress"
  }'
```

### Get All Tasks
```bash
curl -X GET http://localhost:3000/task/getAll \
  -H "Authorization: Bearer <your-jwt-token>"
```

### Update Task
```bash
curl -X PATCH http://localhost:3000/task/update/<task-id> \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <your-jwt-token>" \
  -d '{
    "taskname": "Updated task name",
    "status": "Completed"
  }'
```

## 🚨 Error Handling

The API provides comprehensive error responses:

- **400 Bad Request**: Invalid input data
- **401 Unauthorized**: Invalid credentials or missing token
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server-side errors



### Environment Variables for Production
```bash
MONGODB_URI=mongodb+srv://<username>:<password>@cluster.mongodb.net/rustmanager
JWT_SECRET=your-production-secret-key
PORT=8080

