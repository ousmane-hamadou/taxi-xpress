import SearchForm from "./components/SeachForm";
import styles from "./styles.module.css";

export default async function Home() {
  return (
    <main className="container">
      <div className={styles.page}>
        <p className={styles.question}>Ou voulez-vous allez?</p>

        {/*@ts-expect-error Asycn Component */}
        <SearchForm />
      </div>
    </main>
  );
}
