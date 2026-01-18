import React, { useState } from 'react';

const Badge = ({ children, variant = 'default' }) => {
  const variants = {
    default: 'bg-gray-600 text-gray-200',
    success: 'bg-emerald-500/20 text-emerald-400 border border-emerald-500/30',
    warning: 'bg-yellow-500/20 text-yellow-400 border border-yellow-500/30',
    info: 'bg-blue-500/20 text-blue-400 border border-blue-500/30',
    pending: 'bg-purple-500/20 text-purple-400 border border-purple-500/30',
  };
  
  return (
    <span className={`px-2 py-0.5 rounded text-xs font-medium ${variants[variant]}`}>
      {children}
    </span>
  );
};

const Button = ({ children, variant = 'primary', size = 'md', ...props }) => {
  const variants = {
    primary: 'bg-emerald-500 hover:bg-emerald-600 text-white',
    secondary: 'bg-gray-700 hover:bg-gray-600 text-gray-200',
    ghost: 'bg-transparent hover:bg-gray-700 text-gray-400',
  };
  const sizes = {
    sm: 'px-3 py-1 text-sm',
    md: 'px-4 py-2',
  };
  
  return (
    <button 
      className={`${variants[variant]} ${sizes[size]} rounded font-medium transition-colors`}
      {...props}
    >
      {children}
    </button>
  );
};

const Card = ({ children, className = '' }) => (
  <div className={`bg-gray-800/50 border border-gray-700 rounded-lg ${className}`}>
    {children}
  </div>
);

const TabButton = ({ active, children, onClick }) => (
  <button
    onClick={onClick}
    className={`px-4 py-2 font-medium transition-colors relative ${
      active 
        ? 'text-white' 
        : 'text-gray-400 hover:text-gray-200'
    }`}
  >
    {children}
    {active && (
      <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-emerald-500" />
    )}
  </button>
);

const CreateDealForm = () => {
  const [side, setSide] = useState('buy');
  
  return (
    <Card className="p-6">
      <h2 className="text-lg font-semibold text-white mb-4">Request for quote</h2>
      
      <div className="flex gap-2 mb-6">
        <button 
          onClick={() => setSide('buy')}
          className={`px-4 py-1.5 rounded text-sm font-medium transition-colors ${
            side === 'buy' ? 'bg-emerald-500 text-white' : 'bg-gray-700 text-gray-400'
          }`}
        >
          Buy
        </button>
        <button 
          onClick={() => setSide('sell')}
          className={`px-4 py-1.5 rounded text-sm font-medium transition-colors ${
            side === 'sell' ? 'bg-red-500 text-white' : 'bg-gray-700 text-gray-400'
          }`}
        >
          Sell
        </button>
        <div className="flex-1 bg-gray-700/50 rounded px-3 py-1.5 text-gray-300 text-sm">
          META/USDC
        </div>
      </div>
      
      <div className="space-y-4">
        <div>
          <label className="text-gray-400 text-sm mb-1 block">Buy amount</label>
          <div className="bg-gray-700/50 rounded px-3 py-2 flex justify-between">
            <span className="text-white">4444</span>
            <span className="text-gray-400">META</span>
          </div>
        </div>
        
        <div>
          <label className="text-gray-400 text-sm mb-1 block">Price per META</label>
          <div className="bg-gray-700/50 rounded px-3 py-2 flex justify-between">
            <span className="text-white">444</span>
            <span className="text-gray-400">USDC</span>
          </div>
        </div>
        
        <div>
          <label className="text-gray-400 text-sm mb-1 block">Expires in</label>
          <div className="bg-gray-700/50 rounded px-3 py-2 flex justify-between">
            <span className="text-white">24</span>
            <span className="text-gray-400">hours</span>
          </div>
        </div>
        
        <p className="text-gray-500 text-sm">
          Market makers will respond with quotes. Trades auto-execute when fully filled, or you can execute partial fills manually.
        </p>
        
        <div>
          <label className="text-gray-400 text-sm mb-1 block">Total cost</label>
          <div className="bg-gray-700/50 rounded px-3 py-2 flex justify-between">
            <span className="text-white">1,973,136</span>
            <span className="text-gray-400">USDC</span>
          </div>
        </div>
        
        <Button variant="primary" className="w-full py-3">
          Create Deal
        </Button>
      </div>
    </Card>
  );
};

const YourDeals = () => {
  const deals = [
    { id: 1, type: 'BUY', pair: 'META/USDC', amount: 4444, price: 444, total: '1,973,136', status: 'open', filled: false, expires: '23h 14m' },
    { id: 2, type: 'SELL', pair: 'ETH/USDC', amount: 10, price: 3200, total: '32,000', status: 'partial', filled: true, expires: '5h 42m' },
    { id: 3, type: 'BUY', pair: 'META/USDC', amount: 1000, price: 450, total: '450,000', status: 'executed', filled: true, expires: '-' },
  ];
  
  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="text-gray-400 text-sm border-b border-gray-700">
            <th className="text-left py-3 font-medium">Type</th>
            <th className="text-left py-3 font-medium">Pair</th>
            <th className="text-right py-3 font-medium">Amount</th>
            <th className="text-right py-3 font-medium">Price</th>
            <th className="text-right py-3 font-medium">Total</th>
            <th className="text-center py-3 font-medium">Expires</th>
            <th className="text-center py-3 font-medium">Status</th>
            <th className="text-right py-3 font-medium">Actions</th>
          </tr>
        </thead>
        <tbody>
          {deals.map(deal => (
            <tr key={deal.id} className="border-b border-gray-700/50">
              <td className={`py-3 font-medium ${deal.type === 'BUY' ? 'text-emerald-400' : 'text-red-400'}`}>
                {deal.type}
              </td>
              <td className="py-3 text-gray-200">{deal.pair}</td>
              <td className="py-3 text-right text-gray-200">{deal.amount.toLocaleString()}</td>
              <td className="py-3 text-right text-gray-200">{deal.price}</td>
              <td className="py-3 text-right text-gray-200">{deal.total}</td>
              <td className="py-3 text-center text-gray-400">{deal.expires}</td>
              <td className="py-3 text-center">
                {deal.status === 'open' && <Badge variant="info">open</Badge>}
                {deal.status === 'partial' && <Badge variant="warning">partially filled</Badge>}
                {deal.status === 'executed' && <Badge variant="success">executed</Badge>}
              </td>
              <td className="py-3 text-right">
                {deal.status === 'partial' && (
                  <Button size="sm" variant="primary">Execute</Button>
                )}
                {deal.status === 'open' && (
                  <Button size="sm" variant="ghost">Cancel</Button>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

const OpenMarket = () => {
  const [filter, setFilter] = useState('all');
  
  const deals = [
    { id: 1, pair: 'META/USDC', side: 'BUY', expires: '2h 34m', offers: 3 },
    { id: 2, pair: 'META/USDC', side: 'SELL', expires: '14h 22m', offers: 0 },
    { id: 3, pair: 'ETH/USDC', side: 'BUY', expires: '6h 15m', offers: 7 },
    { id: 4, pair: 'ETH/USDC', side: 'SELL', expires: '1h 05m', offers: 2 },
    { id: 5, pair: 'SOL/USDC', side: 'BUY', expires: '18h 40m', offers: 0 },
  ];
  
  const filteredDeals = filter === 'all' ? deals : deals.filter(d => d.pair.startsWith(filter));
  
  return (
    <div>
      <div className="flex items-center justify-between mb-4">
        <div className="flex gap-2">
          {['all', 'META', 'ETH', 'SOL'].map(f => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`px-3 py-1 rounded text-sm transition-colors ${
                filter === f 
                  ? 'bg-gray-600 text-white' 
                  : 'bg-gray-700/50 text-gray-400 hover:text-gray-200'
              }`}
            >
              {f === 'all' ? 'All Pairs' : f}
            </button>
          ))}
        </div>
        <p className="text-gray-500 text-sm">{filteredDeals.length} active deals</p>
      </div>
      
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="text-gray-400 text-sm border-b border-gray-700">
              <th className="text-left py-3 font-medium">Pair</th>
              <th className="text-left py-3 font-medium">Looking to</th>
              <th className="text-center py-3 font-medium">Expires</th>
              <th className="text-right py-3 font-medium"></th>
            </tr>
          </thead>
          <tbody>
            {filteredDeals.map(deal => (
              <tr key={deal.id} className="border-b border-gray-700/50 hover:bg-gray-700/20">
                <td className="py-3 text-gray-200 font-medium">{deal.pair}</td>
                <td className="py-3">
                  <span className={deal.side === 'BUY' ? 'text-emerald-400' : 'text-red-400'}>
                    {deal.side === 'BUY' ? 'Buy' : 'Sell'}
                  </span>
                  <span className="text-gray-500 ml-1">
                    ({deal.side === 'BUY' ? 'you sell' : 'you buy'})
                  </span>
                </td>
                <td className="py-3 text-center">
                  <span className={deal.expires.startsWith('1h') || deal.expires.startsWith('2h') ? 'text-yellow-400' : 'text-gray-400'}>
                    {deal.expires}
                  </span>
                </td>
                <td className="py-3 text-right">
                  <Button size="sm" variant="secondary">Make Offer</Button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      <div className="mt-4 p-3 bg-gray-700/30 rounded text-sm text-gray-400">
        💡 You won't know the deal creator's price. Submit your best price — if it meets their threshold, your offer passes.
      </div>
    </div>
  );
};

const YourOffers = () => {
  const offers = [
    { id: 1, pair: 'META/USDC', side: 'SELL', yourPrice: 442, submittedAt: '2h ago', status: 'pending', dealStatus: 'active' },
    { id: 2, pair: 'ETH/USDC', side: 'SELL', yourPrice: 3200, submittedAt: '5h ago', status: 'passed', dealStatus: 'executed' },
    { id: 3, pair: 'META/USDC', side: 'BUY', yourPrice: 448, submittedAt: '1d ago', status: 'failed', dealStatus: 'expired' },
    { id: 4, pair: 'SOL/USDC', side: 'SELL', yourPrice: 185, submittedAt: '3h ago', status: 'pending', dealStatus: 'active' },
  ];
  
  return (
    <div>
      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="text-gray-400 text-sm border-b border-gray-700">
              <th className="text-left py-3 font-medium">Pair</th>
              <th className="text-left py-3 font-medium">You</th>
              <th className="text-right py-3 font-medium">Your Price</th>
              <th className="text-center py-3 font-medium">Submitted</th>
              <th className="text-center py-3 font-medium">Deal Status</th>
              <th className="text-center py-3 font-medium">Your Offer</th>
            </tr>
          </thead>
          <tbody>
            {offers.map(offer => (
              <tr key={offer.id} className="border-b border-gray-700/50">
                <td className="py-3 text-gray-200 font-medium">{offer.pair}</td>
                <td className="py-3">
                  <span className={offer.side === 'SELL' ? 'text-red-400' : 'text-emerald-400'}>
                    {offer.side === 'SELL' ? 'Selling' : 'Buying'}
                  </span>
                </td>
                <td className="py-3 text-right text-gray-200">{offer.yourPrice.toLocaleString()}</td>
                <td className="py-3 text-center text-gray-400">{offer.submittedAt}</td>
                <td className="py-3 text-center">
                  {offer.dealStatus === 'active' && <Badge variant="info">active</Badge>}
                  {offer.dealStatus === 'executed' && <Badge variant="success">executed</Badge>}
                  {offer.dealStatus === 'expired' && <Badge variant="default">expired</Badge>}
                </td>
                <td className="py-3 text-center">
                  {offer.status === 'pending' && <Badge variant="pending">pending</Badge>}
                  {offer.status === 'passed' && <Badge variant="success">passed ✓</Badge>}
                  {offer.status === 'failed' && <Badge variant="default">failed</Badge>}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      <div className="mt-4 p-3 bg-gray-700/30 rounded text-sm text-gray-400">
        💡 You'll only know if your offer passed or failed once the deal concludes (executed or expired).
      </div>
    </div>
  );
};

export default function OTCTradingDesk() {
  const [activeTab, setActiveTab] = useState('market');
  
  return (
    <div className="min-h-screen bg-gray-900 p-6">
      <div className="max-w-5xl mx-auto space-y-6">
        {/* Create Deal Form */}
        <div className="max-w-md mx-auto">
          <CreateDealForm />
        </div>
        
        {/* Tabbed Section */}
        <Card>
          <div className="border-b border-gray-700 px-4">
            <div className="flex gap-2">
              <TabButton active={activeTab === 'deals'} onClick={() => setActiveTab('deals')}>
                Your Deals
              </TabButton>
              <TabButton active={activeTab === 'market'} onClick={() => setActiveTab('market')}>
                Open Market
              </TabButton>
              <TabButton active={activeTab === 'offers'} onClick={() => setActiveTab('offers')}>
                Your Offers
              </TabButton>
            </div>
          </div>
          
          <div className="p-4">
            {activeTab === 'deals' && <YourDeals />}
            {activeTab === 'market' && <OpenMarket />}
            {activeTab === 'offers' && <YourOffers />}
          </div>
        </Card>
      </div>
    </div>
  );
}
