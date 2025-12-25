# Session Storage - Design Document

> **Purpose**: Technical design for session statistics persistence
> **Module**: `src/data/`
> **Previous Step**: See `requirements.md` for persistence requirements
> **Related**: See `../../steering/tech.md` for JSON rationale

## Overview

The session storage system persists typing session statistics to JSON files for cross-session tracking and future analytics. It uses the XDG Base Directory specification for Linux/Unix compatibility.

## Data Structures

### Stats File Structure

**JSON Schema:**
```json
{
  "sessions": [
    {
      "timestamp": "2024-01-15T14:30:00Z",
      "lesson_type": "HomeRow-1",
      "wpm": 45.2,
      "accuracy": 96.5,
      "duration": 83
    }
  ]
}
```

**Rust Types:**
```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub sessions: Vec<SessionRecord>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionRecord {
    pub timestamp: DateTime<Utc>,
    pub lesson_type: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub duration: u64,  // seconds
}
```

## Storage Implementation

### File Location

**XDG Base Directory compliance:**
```rust
use std::path::PathBuf;

pub fn get_stats_path() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("typer-cli");

    Ok(config_dir.join("stats.json"))
}
```

**Path resolution:**
- Linux/Unix: `~/.config/typer-cli/stats.json`
- macOS: `~/Library/Application Support/typer-cli/stats.json`
- Windows: `%APPDATA%\typer-cli\stats.json`

### Save Operation

```rust
pub fn save_stats(stats: &Stats) -> Result<(), Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not determine config directory")?
        .join("typer-cli");

    // Create directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;

    let stats_file = config_dir.join("stats.json");
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(stats_file, json)?;

    Ok(())
}
```

**Design Decisions:**
- **Pretty JSON**: Human-readable, easy to debug
- **Create directory**: First run automatically creates config dir
- **Error handling**: Returns Result for graceful degradation

### Load Operation

```rust
pub fn load_stats() -> Result<Stats, Box<dyn Error>> {
    let stats_file = get_stats_path()?;

    if !stats_file.exists() {
        // Return empty stats on first run
        return Ok(Stats {
            sessions: Vec::new(),
        });
    }

    let json = fs::read_to_string(stats_file)?;
    let stats: Stats = serde_json::from_str(&json)?;

    Ok(stats)
}
```

**Design Decisions:**
- **First run handling**: Empty stats if file doesn't exist
- **Non-fatal errors**: App continues even if load fails
- **Lazy loading**: Only load when needed

### Append New Session

```rust
pub fn append_session(session_result: &SessionResult, lesson_type: &str) -> Result<(), Box<dyn Error>> {
    let mut stats = load_stats().unwrap_or_else(|_| Stats {
        sessions: Vec::new(),
    });

    let record = SessionRecord {
        timestamp: Utc::now(),
        lesson_type: lesson_type.to_string(),
        wpm: session_result.wpm,
        accuracy: session_result.accuracy,
        duration: session_result.duration.as_secs(),
    };

    stats.sessions.push(record);
    save_stats(&stats)?;

    Ok(())
}
```

## Data Persistence Design Decisions

### Why JSON over SQLite?

**JSON Advantages:**
- Simple for append-only data
- Human-readable for debugging
- No database engine dependency
- Easy to backup/share
- Sufficient for Phase 1 scale (hundreds of sessions)

**SQLite would be needed if:**
- Complex queries required
- Thousands of sessions
- Concurrent access needed
- Indexing performance critical

### Why Pretty JSON?

**Advantages:**
- Easy to inspect manually
- Better for version control if tracked
- Helps debugging issues

**Disadvantage:**
- Slightly larger file size (~30% overhead)
- Acceptable trade-off for Phase 1

### Append-Only Strategy

**Current approach:**
```rust
// Load entire file → Add session → Save entire file
```

**Why this works for Phase 1:**
- Small file size (<100KB for years of practice)
- Infrequent writes (once per session)
- Simple implementation

**Future optimization (Phase 3+):**
```rust
// Stream append to file without full load
// Periodic compaction/rotation
```

## File System Safety

### Error Handling Strategy

```rust
match save_stats(&stats) {
    Ok(_) => println!("Stats saved successfully"),
    Err(e) => {
        eprintln!("Warning: Could not save stats: {}", e);
        // Continue execution - non-fatal error
    }
}
```

**Design principle:** Stats persistence failures don't crash the app.

### Atomic Writes

**Current:** Direct write (non-atomic)

**Future enhancement:**
```rust
// Write to temporary file
let temp_file = stats_file.with_extension("json.tmp");
fs::write(&temp_file, json)?;

// Atomic rename
fs::rename(temp_file, stats_file)?;
```

### Permissions

**Default behavior:**
- Files created with user-only read/write (0600)
- Directory created with user-only access (0700)
- No privileged operations required

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_stats() {
        let temp_dir = TempDir::new().unwrap();
        let stats_path = temp_dir.path().join("stats.json");

        let stats = Stats {
            sessions: vec![
                SessionRecord {
                    timestamp: Utc::now(),
                    lesson_type: "HomeRow-1".to_string(),
                    wpm: 45.0,
                    accuracy: 95.0,
                    duration: 60,
                }
            ],
        };

        save_stats(&stats, &stats_path).unwrap();
        let loaded = load_stats(&stats_path).unwrap();

        assert_eq!(loaded.sessions.len(), 1);
        assert_eq!(loaded.sessions[0].lesson_type, "HomeRow-1");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let stats_path = temp_dir.path().join("nonexistent.json");

        let stats = load_stats(&stats_path).unwrap();
        assert_eq!(stats.sessions.len(), 0);
    }
}
```

## Future Enhancements (Phase 2+)

### CSV Export

```rust
pub fn export_to_csv(stats: &Stats, path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for session in &stats.sessions {
        writer.serialize(session)?;
    }

    Ok(())
}
```

### Data Migration

When adding new fields:
```rust
#[derive(Serialize, Deserialize)]
pub struct SessionRecord {
    // ... existing fields ...

    #[serde(default)]
    pub per_key_stats: Option<HashMap<char, KeyStats>>,
}
```

The `#[serde(default)]` attribute handles old JSON files gracefully.

## File Locations

- `src/data/stats.rs` - Stats and SessionRecord structures
- `src/data/storage.rs` - Load/save implementations
- `src/data/mod.rs` - Module exports
