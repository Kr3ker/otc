import { SolanaProvider } from "./_providers/SolanaProvider";

export default function OtcLayout({ children }: { children: React.ReactNode }) {
  return <SolanaProvider>{children}</SolanaProvider>;
}
