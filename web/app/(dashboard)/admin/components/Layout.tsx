import React from "react";

export default function Layout({
  topBar,
  children,
}: {
  topBar: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <>
      {topBar}
      <main>{children}</main>
    </>
  );
}
