"use client";

import { prefetchStations } from "./page";
import Provider from "./provider";

prefetchStations();

export default function RootLoyout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html>
      <body style={{ margin: 0 }}>
        <Provider>{children}</Provider>
      </body>
    </html>
  );
}
