import Link from "next/link";
import NavBar from "./components/NavBar";
import "./pico.min.css";
import "./globals.css";

export const metadata = {
  title: "TaxiXpress",
  description: "Taxi bookings app",
};

function NavActions() {
  return (
    <ul>
      <li>
        <Link href="/signin" key="signin">
          Sign In
        </Link>
      </li>
    </ul>
  );
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <NavBar logo={<strong>TaxiXpress</strong>} actions={<NavActions />} />
        {children}
      </body>
    </html>
  );
}
