import { Raleway } from "next/font/google";

export const metadata = {
  title: "Dashboard",
};

const raleway = Raleway({
  weight: "400",
  subsets: ["latin"],
});

export default function DashboardRootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={raleway.className} style={{ margin: 0 }}>
        {children}
      </body>
    </html>
  );
}
