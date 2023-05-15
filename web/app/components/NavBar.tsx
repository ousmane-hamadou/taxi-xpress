import styles from "./NavBar.module.css";

export default function NavBar({
  logo,
  actions,
}: {
  logo: React.ReactNode;
  actions: React.ReactNode;
}) {
  return (
    <nav className="container">
      <ul>
        <li>{logo}</li>
      </ul>
      {actions}
    </nav>
  );
}
