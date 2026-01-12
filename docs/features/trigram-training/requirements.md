# Trigram Training - Requirements

## Overview
The Trigram Training feature generates typing drills based on triplets of characters (trigrams). This helps users improve their typing speed and accuracy by practicing common and challenging character sequences.

## User Stories

### US-1 Generate Trigram Drills
```
WHEN a user selects the Trigram Training category
THE SYSTEM SHALL generate drills containing triplets of characters
AND ensure the drills are randomized for variety.
```

### US-2 Support Common and Rare Trigrams
```
WHEN generating trigram drills
THE SYSTEM SHALL include both common and rare trigrams
TO ensure comprehensive practice.
```

### US-3 Validate Trigram Content
```
WHEN generating trigram drills
THE SYSTEM SHALL ensure all trigrams are valid for the AZERTY layout
AND exclude invalid or unsupported sequences.
```