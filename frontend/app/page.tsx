import Link from "next/link";

export default function HomePage() {
  return (
    <div className="h-screen flex flex-col bg-background">
      {/* Navbar */}
      <nav className="bg-background py-4 shrink-0 border-b border-border">
        <div className="max-w-7xl mx-auto px-6 flex items-center justify-between">
          {/* Logo */}
          <div className="flex items-center gap-2">
            <span className="text-primary text-xl">⬡</span>
            <span className="text-foreground font-semibold text-lg">Veil OTC</span>
          </div>

          {/* Center Nav Links */}
          <div className="flex gap-8 text-sm">
            <a href="#product" className="text-muted-foreground hover:text-foreground transition-colors">
              Product
            </a>
            <a href="#how-it-works" className="text-muted-foreground hover:text-foreground transition-colors">
              How it works
            </a>
            <a href="#security" className="text-muted-foreground hover:text-foreground transition-colors">
              Security
            </a>
            <a href="#faq" className="text-muted-foreground hover:text-foreground transition-colors">
              FAQ
            </a>
          </div>

          {/* CTA Button */}
          <Link
            href="/otc"
            className="bg-primary hover:bg-primary/90 text-primary-foreground px-5 py-2 rounded font-medium text-sm transition-all shadow-[0_0_20px_rgba(249,115,22,0.3)] hover:shadow-[0_0_25px_rgba(249,115,22,0.5)]"
          >
            Open app →
          </Link>
        </div>
      </nav>

      {/* Hero Section */}
      <main className="flex-1 flex flex-col items-center justify-center px-6">
        <div className="max-w-2xl text-center space-y-6">
          <h1 className="text-5xl font-bold text-foreground">
            Private OTC Trading
          </h1>
          <p className="text-xl text-muted-foreground">
            Execute large trades with complete privacy. No slippage, no front-running,
            no information leakage.
          </p>
          <div className="flex gap-4 justify-center pt-4">
            <Link
              href="/otc"
              className="bg-primary hover:bg-primary/90 text-primary-foreground px-6 py-3 rounded font-medium transition-all shadow-[0_0_20px_rgba(249,115,22,0.3)] hover:shadow-[0_0_25px_rgba(249,115,22,0.5)]"
            >
              Start Trading
            </Link>
            <a
              href="#"
              className="bg-secondary hover:bg-secondary/80 text-secondary-foreground px-6 py-3 rounded font-medium transition-colors"
            >
              Learn More
            </a>
          </div>
        </div>
      </main>

      {/* Footer border */}
      <div className="border-t border-border h-3 shrink-0" />
    </div>
  );
}
