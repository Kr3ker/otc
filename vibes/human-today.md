# Human Tasks - 2026-01-23

Things for you to do with your hands.

---

## 1. Local Supabase Setup

### Prerequisites

- Docker Desktop installed and running
- Homebrew (for macOS)

### Steps

**1. Install Supabase CLI**

```bash
brew install supabase/tap/supabase
```

Verify:
```bash
supabase --version
```

**2. Initialize Supabase in project**

From project root (`/Users/pileks/dev/pileks/otc`):

```bash
supabase init
```

This creates a `supabase/` folder with config.

**3. Create the schema migration**

```bash
# Create a new migration file
supabase migration new initial_schema
```

This creates `supabase/migrations/<timestamp>_initial_schema.sql`.

Open that file and paste the schema from `vibes/datamodel/000-initial-draft.md` (Section 3). Here's the full SQL:

```sql
-- deals table
CREATE TABLE deals (
  address TEXT PRIMARY KEY,
  base_mint TEXT NOT NULL,
  quote_mint TEXT NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  allow_partial BOOLEAN NOT NULL,
  status TEXT NOT NULL DEFAULT 'open',
  created_at TIMESTAMPTZ NOT NULL,
  settled_at TIMESTAMPTZ,
  encryption_key BYTEA NOT NULL,
  nonce BYTEA NOT NULL,
  ciphertexts BYTEA NOT NULL,
  settlement_encryption_key BYTEA,
  settlement_nonce BYTEA,
  settlement_ciphertexts BYTEA,
  created_signature TEXT NOT NULL,
  settled_signature TEXT,
  indexed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_deals_status ON deals(status);
CREATE INDEX idx_deals_mints ON deals(base_mint, quote_mint);
CREATE INDEX idx_deals_expires_at ON deals(expires_at);

-- offers table
CREATE TABLE offers (
  address TEXT PRIMARY KEY,
  deal_address TEXT NOT NULL REFERENCES deals(address),
  offer_index INT NOT NULL,
  status TEXT NOT NULL DEFAULT 'open',
  submitted_at TIMESTAMPTZ NOT NULL,
  encryption_key BYTEA NOT NULL,
  nonce BYTEA NOT NULL,
  ciphertexts BYTEA NOT NULL,
  settlement_encryption_key BYTEA,
  settlement_nonce BYTEA,
  settlement_ciphertexts BYTEA,
  created_signature TEXT NOT NULL,
  settled_signature TEXT,
  indexed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_offers_deal ON offers(deal_address);
CREATE INDEX idx_offers_status ON offers(status);

-- optional: raw_events audit trail
CREATE TABLE raw_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  signature TEXT NOT NULL,
  slot BIGINT NOT NULL,
  block_time TIMESTAMPTZ,
  event_name TEXT NOT NULL,
  raw_data BYTEA NOT NULL,
  indexed_at TIMESTAMPTZ DEFAULT NOW(),
  UNIQUE(signature, event_name)
);

CREATE INDEX idx_raw_events_slot ON raw_events(slot);
CREATE INDEX idx_raw_events_event_name ON raw_events(event_name);

-- Enable Realtime for live updates
ALTER PUBLICATION supabase_realtime ADD TABLE deals;
ALTER PUBLICATION supabase_realtime ADD TABLE offers;
```

**4. Start local Supabase**

```bash
supabase start
```

First run downloads Docker images (~2-3 min). When done, you'll see:

```
Started supabase local development setup.

         API URL: http://localhost:54321
          DB URL: postgresql://postgres:postgres@localhost:54322/postgres
      Studio URL: http://localhost:54323
        ...
```

**5. Apply migrations**

```bash
supabase db reset
```

This drops and recreates the database with your migrations.

**6. Open Studio**

Go to http://localhost:54323 in your browser. You should see:
- Table Editor with `deals`, `offers`, `raw_events` tables
- SQL Editor for ad-hoc queries

**7. Generate TypeScript types**

```bash
supabase gen types typescript --local > frontend/app/otc/_lib/database.types.ts
```

This generates type-safe interfaces matching your schema.

---

## 2. Verify Setup

In Supabase Studio (http://localhost:54323):

1. Go to **Table Editor**
2. Click on `deals` table
3. Try inserting a test row (will need valid data for NOT NULL fields)
4. Delete the test row

Or via SQL Editor:
```sql
-- Check tables exist
SELECT table_name FROM information_schema.tables
WHERE table_schema = 'public';

-- Check Realtime is enabled
SELECT * FROM pg_publication_tables
WHERE pubname = 'supabase_realtime';
```

---

## 3. Useful Commands

```bash
supabase start      # Start local stack
supabase stop       # Stop (keeps data)
supabase db reset   # Reset DB and apply migrations
supabase status     # Show URLs and keys
supabase migration list  # List migrations
```

---

## 4. Environment Variables

After setup, you'll need these for the indexer/frontend:

```bash
# Local development
SUPABASE_URL=http://localhost:54321
SUPABASE_ANON_KEY=<shown in supabase start output>
SUPABASE_SERVICE_KEY=<shown in supabase start output>
```

The anon key is for frontend (public, RLS-restricted).
The service key is for indexer (bypasses RLS, keep secret).

---

## When You're Done

Let me know and we can:
1. Add seed data for testing
2. Set up the indexer to populate it
3. Wire up the frontend to read from it
