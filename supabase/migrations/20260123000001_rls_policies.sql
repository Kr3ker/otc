-- RLS Policies for OTC Platform
-- Frontend (anon): read-only access
-- Indexer (service_role): bypasses RLS, full access

--------------------------------------------------------------------------------
-- ENABLE RLS
--------------------------------------------------------------------------------

ALTER TABLE deals ENABLE ROW LEVEL SECURITY;
ALTER TABLE offers ENABLE ROW LEVEL SECURITY;
ALTER TABLE raw_events ENABLE ROW LEVEL SECURITY;

--------------------------------------------------------------------------------
-- DEALS POLICIES
--------------------------------------------------------------------------------

-- Anyone can read deals (market data is public, encrypted fields can't be decrypted)
CREATE POLICY "deals_select_public"
  ON deals
  FOR SELECT
  TO anon, authenticated
  USING (true);

--------------------------------------------------------------------------------
-- OFFERS POLICIES
--------------------------------------------------------------------------------

-- Anyone can read offers (same reasoning as deals)
CREATE POLICY "offers_select_public"
  ON offers
  FOR SELECT
  TO anon, authenticated
  USING (true);

--------------------------------------------------------------------------------
-- RAW_EVENTS POLICIES
--------------------------------------------------------------------------------

-- Anyone can read raw events (audit trail, public data)
CREATE POLICY "raw_events_select_public"
  ON raw_events
  FOR SELECT
  TO anon, authenticated
  USING (true);

--------------------------------------------------------------------------------
-- NOTES
--------------------------------------------------------------------------------

-- No INSERT/UPDATE/DELETE policies for anon/authenticated roles.
-- This means only service_role (used by indexer) can write.
-- service_role bypasses RLS entirely.
