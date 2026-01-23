"use client";

import { useState, useEffect } from "react";
import { type Deal, type MarketDeal, type Offer } from "./_lib/types";
import { MOCK_DEALS, MOCK_MARKET_DEALS, MOCK_OFFERS } from "./_lib/constants";
import { FAQPanel } from "./_components/FAQPanel";
import { TabNavigation, type TabId } from "./_components/TabNavigation";
import { Navbar } from "./_components/Navbar";
import { DealsTable } from "./_components/DealsTable";
import { MarketTable } from "./_components/MarketTable";
import { OffersTable } from "./_components/OffersTable";
import { DealDetails } from "./_components/DealDetails";
import { MakeOfferForm } from "./_components/MakeOfferForm";
import { CreateDealForm } from "./_components/CreateDealForm";

export default function OTCPage() {
  // Tab state
  const [activeTab, setActiveTab] = useState<TabId>("market");
  const [deals, setDeals] = useState<Deal[]>(MOCK_DEALS);
  const [marketDeals] = useState<MarketDeal[]>(MOCK_MARKET_DEALS);
  const [offers] = useState<Offer[]>(MOCK_OFFERS);
  const [pairFilter, setPairFilter] = useState<string>("all");

  // Selected market deal for expanded view
  const [selectedMarketDeal, setSelectedMarketDeal] = useState<MarketDeal | null>(null);

  // Real-time countdown state for DealDetails
  const [, setTick] = useState(0);

  useEffect(() => {
    if (!selectedMarketDeal) return;
    const interval = setInterval(() => setTick((t) => t + 1), 1000);
    return () => clearInterval(interval);
  }, [selectedMarketDeal]);

  // Filter market deals
  const filteredMarketDeals =
    pairFilter === "all"
      ? marketDeals
      : marketDeals.filter((d) => d.pair.startsWith(pairFilter));

  // Handle row click
  const handleMarketDealClick = (deal: MarketDeal) => {
    setSelectedMarketDeal(deal);
  };

  // Collapse back to table
  const handleCollapse = () => {
    setSelectedMarketDeal(null);
  };

  // Handle new deal created
  const handleDealCreated = (newDeal: Deal) => {
    setDeals((prev) => [newDeal, ...prev]);
    setActiveTab("deals");
  };

  // Handle offer placed
  const handleOfferPlaced = () => {
    setSelectedMarketDeal(null);
    setActiveTab("offers");
  };

  return (
    <div className="h-screen flex flex-col bg-background">
      {/* Navbar */}
      <Navbar />

      {/* Three-column layout */}
      <div className="flex flex-1 min-h-0">
        {/* Left Panel - Create Deal or Make Offer form */}
        <div className="w-[440px] shrink-0 border-r border-border p-4 overflow-y-auto">
          <div className="bg-card/50 border border-border rounded-lg p-4">
            {/* Show Make Offer form when deal is selected, otherwise Create Deal */}
            {selectedMarketDeal ? (
              <MakeOfferForm
                deal={selectedMarketDeal}
                onOfferPlaced={handleOfferPlaced}
                onClose={handleCollapse}
              />
            ) : (
              <CreateDealForm onDealCreated={handleDealCreated} />
            )}
          </div>
        </div>

        {/* Center Panel - Tables or Deal Details */}
        <div className="flex-1 p-4 overflow-y-auto">
          <div className="bg-card/50 border border-border rounded-lg">
            {/* Show Deal Details when a market deal is selected */}
            {selectedMarketDeal ? (
              <DealDetails deal={selectedMarketDeal} onBack={handleCollapse} />
            ) : (
              <>
                {/* Tab Navigation */}
                <TabNavigation activeTab={activeTab} onTabChange={setActiveTab} />

                {/* Tab Content */}
                <div className="p-4">
                  {activeTab === "deals" && <DealsTable deals={deals} />}
                  {activeTab === "market" && (
                    <MarketTable
                      deals={marketDeals}
                      filteredDeals={filteredMarketDeals}
                      pairFilter={pairFilter}
                      onPairFilterChange={setPairFilter}
                      onDealClick={handleMarketDealClick}
                    />
                  )}
                  {activeTab === "offers" && <OffersTable offers={offers} />}
                </div>
              </>
            )}
          </div>
        </div>

        {/* Right Panel - Negotiation */}
        <FAQPanel />
      </div>

      {/* Footer border */}
      <div className="border-t border-border h-3 shrink-0" />
    </div>
  );
}
