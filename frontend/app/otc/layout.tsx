import { SolanaProvider } from "./_providers/SolanaProvider";
import { DerivedKeysProvider } from "./_providers/DerivedKeysProvider";

export default function OtcLayout({ children }: { children: React.ReactNode }) {
  return (
    <SolanaProvider>
      <DerivedKeysProvider>{children}</DerivedKeysProvider>
    </SolanaProvider>
  );
}
