# Mirabel TUI

A Terminal User Interface for the Mirabel AI Software Developer system. This TUI provides developers with a command-line native interface for interacting with Mirabel's capabilities, offering a keyboard-driven alternative to the web-based frontend.

## Overview

Mirabel is an AI software developer with a modular architecture including Communication, Orchestrator, Planner, Researcher, Tool Integration, Memory Management, Programmer, Machine, and Browser modules. This TUI complements the existing web-based frontend by providing developers with a terminal-native interface that integrates seamlessly with existing developer workflows.

## Core Requirements

### Primary Use Cases
1. **Chat Interface** - Direct communication with Mirabel agents
2. **Project Management** - Workspace/session management 
3. **Task Monitoring** - Real-time plan execution tracking
4. **System Status** - Health monitoring of all modules
5. **Terminal Integration** - Native shell experience for developers

### Target Users
- CLI-focused developers
- System administrators 
- DevOps engineers
- Users working in headless environments
- Developers preferring keyboard-driven workflows

## Architecture Design

### Technology Stack
- **Language**: Rust (consistent with backend)
- **TUI Framework**: `ratatui` (most mature Rust TUI library)
- **Dependencies**:
  - `crossterm` - cross-platform terminal handling
  - `tokio` - async runtime (matches backend)
  - `serde` - JSON serialization (existing in project)
  - `reqwest` - HTTP client for backend API
  - `uuid` - session management

### Component Structure

```
tui/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── app/
│   │   ├── mod.rs             # Main app state & event loop
│   │   ├── config.rs          # Configuration management
│   │   └── events.rs          # Event handling system
│   ├── ui/
│   │   ├── mod.rs             # UI component registry
│   │   ├── layout.rs          # Main layout management
│   │   ├── chat.rs            # Chat interface widget
│   │   ├── workspace.rs       # Workspace browser
│   │   ├── tasks.rs           # Task/plan monitoring
│   │   ├── status.rs          # System status dashboard
│   │   ├── terminal.rs        # Embedded terminal
│   │   └── help.rs            # Help/keybinding display
│   ├── backend/
│   │   ├── mod.rs             # Backend API client
│   │   ├── client.rs          # HTTP client wrapper
│   │   ├── auth.rs            # Authentication handling
│   │   └── websocket.rs       # Real-time communication
│   ├── models/
│   │   ├── mod.rs             # Data models
│   │   ├── session.rs         # Session management
│   │   ├── workspace.rs       # Workspace data
│   │   └── message.rs         # Chat messages
│   └── utils/
│       ├── mod.rs             # Utility functions
│       ├── keybindings.rs     # Key mapping
│       └── colors.rs          # Theme management
├── Cargo.toml
└── README.md
```

### UI Layout Design

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ Mirabel TUI v0.1.0                           [Workspace: my-project] [●] Connected │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│ ┌─ Chat ──────────────────────────┐ ┌─ Tasks ─────────────────────────────┐ │
│ │ You: Add error handling to...   │ │ ✓ Database Schema Setup             │ │
│ │                                 │ │ ⚡ Backend API Implementation       │ │
│ │ Mirabel: I'll help you add...   │ │   ├─ ✓ History Route               │ │
│ │ Let me analyze your codebase... │ │   ├─ ⚡ Query Service              │ │
│ │                                 │ │   └─ ⏸ CSV Export                 │ │
│ │ Analysis complete. Here's my... │ │ ⏸ Frontend Implementation          │ │
│ │                                 │ │ ⏸ Testing & Validation             │ │
│ │ > _                            │ │                                     │ │
│ └─────────────────────────────────┘ └─────────────────────────────────────┘ │
│                                                                             │
│ ┌─ System Status ─────────────────┐ ┌─ Terminal ──────────────────────────┐ │
│ │ Backend: ●  LLM: ●  Browser: ●  │ │ $ npm test                          │ │
│ │ Memory: 45%  CPU: 12%           │ │ > test-project@1.0.0 test           │ │
│ │ Active Tasks: 3                 │ │ > jest                              │ │
│ │                                 │ │                                     │ │
│ │                                 │ │ PASS src/components/Button.test.js  │ │
│ │                                 │ │ ✓ renders without crashing          │ │ │
│ │                                 │ │                                     │ │
│ └─────────────────────────────────┘ └─────────────────────────────────────┘ │
│                                                                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ [Tab] Switch Panel | [Ctrl+C] Quit | [F1] Help | [F2] Settings              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Key Features

### 1. Multi-Panel Interface
- **Chat Panel**: Real-time conversation with Mirabel
- **Task Panel**: Hierarchical plan visualization with progress
- **Status Panel**: System health and resource monitoring  
- **Terminal Panel**: Embedded terminal for command execution
- **Tab-based navigation** between panels

### 2. Real-time Updates
- WebSocket integration for live chat and task updates
- Progress bars and spinners for long-running operations
- Color-coded status indicators (green/yellow/red)

### 3. Keyboard-Driven UX
```
General:
- Tab/Shift+Tab: Navigate panels
- Ctrl+C: Quit application
- F1: Toggle help overlay
- F2: Settings/configuration

Chat Panel:
- Enter: Send message
- Up/Down: Message history
- Ctrl+L: Clear chat
- Ctrl+U: Upload file

Task Panel:
- Space: Expand/collapse task
- Enter: View task details
- R: Retry failed task
- P: Pause/resume task

Terminal Panel:
- All standard terminal keybindings
- Ctrl+Shift+C: Copy
- Ctrl+Shift+V: Paste
```

### 4. Configuration System
```toml
# ~/.config/mirabel/config.toml
[connection]
backend_url = "http://localhost:8080"
websocket_url = "ws://localhost:8080/ws"

[ui]
theme = "dark"  # dark, light, auto
refresh_rate = 250  # milliseconds
max_chat_history = 1000

[keybindings]
quit = "Ctrl+C"
help = "F1"
clear_chat = "Ctrl+L"

[terminal]
shell = "/bin/bash"
scrollback = 10000
```

## Implementation Phases

### Phase 1: Core Infrastructure
1. **Project Setup**
   - Create `tui/` directory in workspace
   - Add `ratatui` and dependencies to `Cargo.toml`
   - Implement basic app structure and event loop

2. **Backend Integration**
   - HTTP client for REST API communication
   - Authentication token management
   - Basic error handling and reconnection logic

3. **Basic UI Layout**
   - Terminal setup with `crossterm`
   - Multi-panel layout implementation
   - Basic navigation between panels

### Phase 2: Chat Interface
1. **Chat Widget Implementation**
   - Message rendering with syntax highlighting
   - Input handling with history
   - Real-time message updates via WebSocket

2. **WebSocket Integration**
   - Connection management
   - Message protocol implementation
   - Automatic reconnection handling

### Phase 3: Task Monitoring
1. **Task Panel**
   - Hierarchical task tree visualization
   - Progress indicators and status icons
   - Task detail views and controls

2. **Real-time Updates**
   - Live task status synchronization
   - Plan execution progress tracking

### Phase 4: System Integration
1. **Status Dashboard**
   - System health monitoring
   - Resource usage displays
   - Module status tracking

2. **Terminal Integration**
   - Embedded terminal widget
   - Command execution in workspace context
   - Output capture and display

### Phase 5: Polish & Features
1. **Configuration System**
   - Settings file management
   - Theme support (dark/light)
   - Keybinding customization

2. **User Experience**
   - Help system and tutorials
   - Error handling and user feedback
   - Performance optimization

## Integration Points

### Backend API Extensions
```rust
// New TUI-specific endpoints
GET  /api/tui/status          // System health
GET  /api/tui/tasks/tree      // Hierarchical task view  
POST /api/tui/terminal/exec   // Command execution
WS   /api/tui/events          // Real-time event stream
```

### Workspace Integration
- Automatic workspace detection from current directory
- Project-specific configuration and state
- Integration with existing session management

### Docker Environment
- TUI binary included in development containers
- Shared volume for workspace access
- Environment variable configuration

## Benefits

1. **Developer Productivity**: Native terminal experience for CLI-focused developers
2. **Resource Efficiency**: Lighter than web browser for simple interactions  
3. **Remote Development**: Works seamlessly over SSH connections
4. **Integration**: Natural fit with existing developer workflows
5. **Accessibility**: Better for users who prefer keyboard navigation

## Technical Considerations

1. **Terminal Compatibility**: Support for various terminal emulators
2. **Unicode Support**: Proper handling of emojis and special characters
3. **Color Support**: Graceful degradation for limited color terminals
4. **Resize Handling**: Dynamic layout adjustment on terminal resize
5. **Performance**: Efficient rendering to avoid flicker/lag

## Getting Started

### Prerequisites
- Rust 1.70+ with Cargo
- Access to Mirabel backend instance
- Terminal with Unicode and color support

### Installation
```bash
# From the Mirabel project root
cd tui
cargo build --release

# Or install globally
cargo install --path .
```

### Configuration
1. Copy example config: `cp config.example.toml ~/.config/mirabel/config.toml`
2. Edit configuration file with your backend URL
3. Run: `mirabel-tui`

### Development
```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

## Contributing

1. Follow the existing code style and patterns
2. Add tests for new functionality
3. Update documentation as needed
4. Ensure terminal compatibility across platforms

## License

Same as parent Mirabel project.