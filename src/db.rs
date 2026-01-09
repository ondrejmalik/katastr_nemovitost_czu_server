use anyhow::Result;
use deadpool_postgres::Pool;
use rust_decimal::Decimal;

use crate::models::*;

pub async fn query_part_a(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<MajitelPartA>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut owners = Vec::new();
    for row in rows {
        let owner = MajitelPartA {
            jmeno: row.try_get("jmeno")?,
            prijmeni: row.try_get("prijmeni")?,
            bydliste: row.try_get("bydliste")?,
            podil_setin: row.try_get::<_, i32>("podil_setin")? as i64,
        };
        owners.push(owner);
    }

    Ok(owners)
}

pub async fn query_majitel_custom(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<Majitel>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = Majitel {
            id: row.try_get("id")?,
            jmeno: row.try_get("jmeno")?,
            prijmeni: row.try_get("prijmeni")?,
            titul: row.try_get("titul")?,
            bydliste: row.try_get("bydliste")?,
            rodne_cislo: row.try_get("rodne_cislo")?,
            ico: row.try_get("ico")?,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn get_majitel(pool: Pool) -> Result<Vec<Majitel>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT id, jmeno, prijmeni, titul, bydliste, rodne_cislo, ico FROM majitel",
            &[],
        )
        .await?;

    let mut items = Vec::new();
    for row in rows {
        let item = Majitel {
            id: row.try_get("id")?,
            jmeno: row.try_get("jmeno")?,
            prijmeni: row.try_get("prijmeni")?,
            titul: row.try_get("titul")?,
            bydliste: row.try_get("bydliste")?,
            rodne_cislo: row.try_get("rodne_cislo")?,
            ico: row.try_get("ico")?,
        };
        items.push(item);
    }
    Ok(items)
}

pub async fn update_majitel(pool: Pool, majitel: Majitel) -> Result<u64> {
    let client = pool.get().await?;
    let stmt = "UPDATE majitel SET jmeno = $2, prijmeni = $3, titul = $4, bydliste = $5, rodne_cislo = $6, ico = $7 WHERE id = $1";
    let rows_affected = client
        .execute(
            stmt,
            &[
                &majitel.id,
                &majitel.jmeno,
                &majitel.prijmeni,
                &majitel.titul,
                &majitel.bydliste,
                &majitel.rodne_cislo,
                &majitel.ico,
            ],
        )
        .await?;
    Ok(rows_affected)
}

pub async fn create_majitel(pool: Pool, majitel: NewMajitel) -> Result<u64> {
    let client = pool.get().await?;
    let stmt = "INSERT INTO majitel (jmeno, prijmeni, titul, bydliste, rodne_cislo, ico) VALUES ($1, $2, $3, $4, $5, $6)";
    let rows_affected = client
        .execute(
            stmt,
            &[
                &majitel.jmeno,
                &majitel.prijmeni,
                &majitel.titul,
                &majitel.bydliste,
                &majitel.rodne_cislo,
                &majitel.ico,
            ],
        )
        .await?;
    Ok(rows_affected)
}

pub async fn delete_majitel(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let stmt = "DELETE FROM majitel WHERE id = $1";
    let rows_affected = client.execute(stmt, &[&id]).await?;
    Ok(rows_affected)
}

pub async fn query_part_b(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<Parcela>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut parcels = Vec::new();
    for row in rows {
        let parcel = Parcela {
            parcelni_cislo: row.try_get::<_, i32>("parcelni_cislo")? as i64,
            je_stavebni: row.try_get("je_stavebni")?,
            ulice: row.try_get("ulice")?,
            cislo_popisne: row.try_get("cislo_popisne")?,
            nazev_ku: row.try_get("nazev_ku")?,
        };
        parcels.push(parcel);
    }

    Ok(parcels)
}

pub async fn query_part_b_parcela(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<ParcelaB>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut parcels = Vec::new();
    for row in rows {
        let parcel = ParcelaB {
            popis: row.try_get("popis")?,
            datum_zrizeni: row.try_get("datum_zrizeni")?,
            datum_pravnich_ucinku: row.try_get("datum_pravnich_ucinku")?,
            je_stavebni_opravnena: row.try_get("je_stavebni_opravnena")?,
            parcelni_cislo_opravnena: row.try_get::<_, i32>("parcelni_cislo_opravnena")? as i64,
            cast_parcely_opravnena: row.try_get::<_, i32>("cast_parcely_opravnena")? as i64,
            je_stavebni_povinna: row.try_get("je_stavebni_povinna")?,
            parcelni_cislo_povinna: row.try_get::<_, i32>("parcelni_cislo_povinna")? as i64,
            cast_parcely_povinna: row.try_get::<_, i32>("cast_parcely_povinna")? as i64,
        };
        parcels.push(parcel);
    }

    Ok(parcels)
}

pub async fn query_part_b_majitel(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<MajitelB>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = MajitelB {
            popis: row.try_get("popis")?,
            datum_zrizeni: row.try_get("datum_zrizeni")?,
            datum_pravnich_ucinku: row.try_get("datum_pravnich_ucinku")?,
            je_stavebni_opravnena: row.try_get("je_stavebni_opravnena")?,
            parcelni_cislo_opravnena: row.try_get::<_, i32>("parcelni_cislo_opravnena")? as i64,
            cast_parcely_opravnena: row.try_get::<_, i32>("cast_parcely_opravnena")? as i64,
            jmeno_povinny: row.try_get("jmeno_povinny")?,
            prijmeni_povinny: row.try_get("prijmeni_povinny")?,
            titul_povinny: row.try_get("titul_povinny")?,
            rodne_cislo_povinny: row.try_get("rodne_cislo_povinny")?,
            ico_povinny: row.try_get("ico_povinny")?,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_part_c(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<PartC>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = PartC {
            popis: row.try_get("popis")?,
            datum_zrizeni: row.try_get("datum_zrizeni")?,
            datum_pravnich_ucinku: row.try_get("datum_pravnich_ucinku")?,
            je_stavebni_opravnena: row.try_get("je_stavebni_opravnena")?,
            parcelni_cislo_opravnena: row.try_get::<_, i32>("parcelni_cislo_opravnena")? as i64,
            cast_parcely_opravnena: row.try_get::<_, i32>("cast_parcely_opravnena")? as i64,
            je_stavebni_povinna: row.try_get("je_stavebni_povinna")?,
            parcelni_cislo_povinna: row.try_get::<_, i32>("parcelni_cislo_povinna")? as i64,
            cast_parcely_povinna: row.try_get::<_, i32>("cast_parcely_povinna")? as i64,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_part_d(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<PartD>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = PartD {
            je_stavebni: row.try_get("je_stavebni")?,
            parcelni_cislo: row.try_get::<_, i32>("parcelni_cislo")? as i64,
            cast_parcely: row.try_get::<_, i32>("cast_parcely")? as i64,
            nazev_katastralniho_uzemi: row.try_get("nazev_katastralniho_uzemi")?,
            typ_rizeni_zkratka: row.try_get("typ_rizeni_zkratka")?,
            cislo_rizeni: row.try_get::<_, i32>("cislo_rizeni")? as i64,
            rok_rizeni: row.try_get::<_, i32>("rok_rizeni")? as i64,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_part_f(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<PartF>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = PartF {
            je_stavebni: row.try_get("je_stavebni")?,
            parcelni_cislo: row.try_get::<_, i32>("parcelni_cislo")? as i64,
            cast_parcely: row.try_get::<_, i32>("cast_parcely")? as i64,
            hodnota: row.try_get::<_, Option<i32>>("hodnota")?.map(|v| v as i64),
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_parcela(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<FindParcela>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = FindParcela {
            je_stavebni: row.try_get("je_stavebni")?,
            parcelni_cislo: row.try_get::<_, i32>("parcelni_cislo")? as i64,
            cast_parcely: row.try_get::<_, i32>("cast_parcely")? as i64,
            vymera_metru_ctverecnich: row
                .try_get::<_, Option<Decimal>>("vymera_metru_ctverecnich")?,
            ulice: row.try_get("ulice")?,
            cislo_popisne: row.try_get("cislo_popisne")?,
            hodnota: row.try_get::<_, Option<i32>>("hodnota")?.map(|v| v as i64),
            cislo_lv: row.try_get::<_, i32>("cislo_lv")? as i64,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_rizeni_predmet_poznamka(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<RizeniPredmetPoznamka>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = RizeniPredmetPoznamka {
            predmet: row.try_get("predmet")?,
            poznamka: row.try_get("poznamka")?,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_rizeni_ucastnici(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<RizeniUcastnik>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = RizeniUcastnik {
            typ_ucastnika: row.try_get("typ_ucastnika")?,
            ucastnik_jmeno: row.try_get("ucastnik_jmeno")?,
        };
        items.push(item);
    }

    Ok(items)
}

pub async fn query_rizeni_operace(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<RizeniOperace>> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut items = Vec::new();
    for row in rows {
        let item = RizeniOperace {
            operace_popis: row.try_get("operace_popis")?,
            operace_datum: row.try_get("operace_datum")?,
        };
        items.push(item);
    }

    Ok(items)
}

// --- Kraj ---
pub async fn get_kraj(pool: Pool) -> Result<Vec<Kraj>> {
    let client = pool.get().await?;
    let rows = client.query("SELECT id, nazev FROM kraj", &[]).await?;
    Ok(rows
        .iter()
        .map(|row| Kraj {
            id: row.get(0),
            nazev: row.get(1),
        })
        .collect())
}

pub async fn create_kraj(pool: Pool, item: NewKraj) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("INSERT INTO kraj (nazev) VALUES ($1)", &[&item.nazev])
        .await?;
    Ok(rows)
}

pub async fn update_kraj(pool: Pool, item: Kraj) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE kraj SET nazev = $2 WHERE id = $1",
            &[&item.id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_kraj(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM kraj WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- Okres ---
pub async fn get_okres(pool: Pool) -> Result<Vec<Okres>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, kraj_id, nazev FROM okres", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| Okres {
            id: row.get(0),
            kraj_id: row.get(1),
            nazev: row.get(2),
        })
        .collect())
}

pub async fn create_okres(pool: Pool, item: NewOkres) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO okres (kraj_id, nazev) VALUES ($1, $2)",
            &[&item.kraj_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn update_okres(pool: Pool, item: Okres) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE okres SET kraj_id = $2, nazev = $3 WHERE id = $1",
            &[&item.id, &item.kraj_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_okres(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM okres WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- Obec ---
pub async fn get_obec(pool: Pool) -> Result<Vec<Obec>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, okres_id, nazev FROM obec", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| Obec {
            id: row.get(0),
            okres_id: row.get(1),
            nazev: row.get(2),
        })
        .collect())
}

pub async fn create_obec(pool: Pool, item: NewObec) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO obec (okres_id, nazev) VALUES ($1, $2)",
            &[&item.okres_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn update_obec(pool: Pool, item: Obec) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE obec SET okres_id = $2, nazev = $3 WHERE id = $1",
            &[&item.id, &item.okres_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_obec(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM obec WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- KatastralniUzemi ---
pub async fn get_katastralni_uzemi(pool: Pool) -> Result<Vec<KatastralniUzemi>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, obec_id, nazev FROM katastralni_uzemi", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| KatastralniUzemi {
            id: row.get(0),
            obec_id: row.get(1),
            nazev: row.get(2),
        })
        .collect())
}

pub async fn create_katastralni_uzemi(pool: Pool, item: NewKatastralniUzemi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO katastralni_uzemi (obec_id, nazev) VALUES ($1, $2)",
            &[&item.obec_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn update_katastralni_uzemi(pool: Pool, item: KatastralniUzemi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE katastralni_uzemi SET obec_id = $2, nazev = $3 WHERE id = $1",
            &[&item.id, &item.obec_id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_katastralni_uzemi(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM katastralni_uzemi WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- Bpej ---
pub async fn get_bpej(pool: Pool) -> Result<Vec<Bpej>> {
    let client = pool.get().await?;
    let rows = client.query("SELECT id, hodnota FROM bpej", &[]).await?;
    Ok(rows
        .iter()
        .map(|row| Bpej {
            id: row.get(0),
            hodnota: row.get(1),
        })
        .collect())
}

pub async fn create_bpej(pool: Pool, item: NewBpej) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("INSERT INTO bpej (hodnota) VALUES ($1)", &[&item.hodnota])
        .await?;
    Ok(rows)
}

pub async fn update_bpej(pool: Pool, item: Bpej) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE bpej SET hodnota = $2 WHERE id = $1",
            &[&item.id, &item.hodnota],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_bpej(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM bpej WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- TypRizeni ---
pub async fn get_typ_rizeni(pool: Pool) -> Result<Vec<TypRizeni>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, nazev, zkratka FROM typ_rizeni", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| TypRizeni {
            id: row.get(0),
            nazev: row.get(1),
            zkratka: row.get(2),
        })
        .collect())
}

pub async fn create_typ_rizeni(pool: Pool, item: NewTypRizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO typ_rizeni (nazev, zkratka) VALUES ($1, $2)",
            &[&item.nazev, &item.zkratka],
        )
        .await?;
    Ok(rows)
}

pub async fn update_typ_rizeni(pool: Pool, item: TypRizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE typ_rizeni SET nazev = $2, zkratka = $3 WHERE id = $1",
            &[&item.id, &item.nazev, &item.zkratka],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_typ_rizeni(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM typ_rizeni WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- TypOperace ---
pub async fn get_typ_operace(pool: Pool) -> Result<Vec<TypOperace>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, popis FROM typ_operace", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| TypOperace {
            id: row.get(0),
            popis: row.get(1),
        })
        .collect())
}

pub async fn create_typ_operace(pool: Pool, item: NewTypOperace) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO typ_operace (popis) VALUES ($1)",
            &[&item.popis],
        )
        .await?;
    Ok(rows)
}

pub async fn update_typ_operace(pool: Pool, item: TypOperace) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE typ_operace SET popis = $2 WHERE id = $1",
            &[&item.id, &item.popis],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_typ_operace(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM typ_operace WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- TypUcastnika ---
pub async fn get_typ_ucastnika(pool: Pool) -> Result<Vec<TypUcastnika>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, nazev FROM typ_ucastnika", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| TypUcastnika {
            id: row.get(0),
            nazev: row.get(1),
        })
        .collect())
}

pub async fn create_typ_ucastnika(pool: Pool, item: NewTypUcastnika) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO typ_ucastnika (nazev) VALUES ($1)",
            &[&item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn update_typ_ucastnika(pool: Pool, item: TypUcastnika) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE typ_ucastnika SET nazev = $2 WHERE id = $1",
            &[&item.id, &item.nazev],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_typ_ucastnika(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM typ_ucastnika WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- UcastnikRizeni ---
pub async fn get_ucastnik_rizeni(pool: Pool) -> Result<Vec<UcastnikRizeni>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT id, jmeno FROM ucastnik_rizeni", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| UcastnikRizeni {
            id: row.get(0),
            jmeno: row.get(1),
        })
        .collect())
}

pub async fn create_ucastnik_rizeni(pool: Pool, item: NewUcastnikRizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO ucastnik_rizeni (jmeno) VALUES ($1)",
            &[&item.jmeno],
        )
        .await?;
    Ok(rows)
}

pub async fn update_ucastnik_rizeni(pool: Pool, item: UcastnikRizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE ucastnik_rizeni SET jmeno = $2 WHERE id = $1",
            &[&item.id, &item.jmeno],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_ucastnik_rizeni(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM ucastnik_rizeni WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- ListVlastnictvi ---
pub async fn get_list_vlastnictvi(pool: Pool) -> Result<Vec<ListVlastnictvi>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT id, katastralni_uzemi_id, cislo_lv, vlastnicky_hash FROM list_vlastnictvi",
            &[],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|row| ListVlastnictvi {
            id: row.get(0),
            katastralni_uzemi_id: row.get(1),
            cislo_lv: row.get(2),
            vlastnicky_hash: row.get(3),
        })
        .collect())
}

pub async fn create_list_vlastnictvi(pool: Pool, item: NewListVlastnictvi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO list_vlastnictvi (katastralni_uzemi_id, cislo_lv, vlastnicky_hash) VALUES ($1, $2, $3)",
        &[&item.katastralni_uzemi_id, &item.cislo_lv, &item.vlastnicky_hash]
    ).await?;
    Ok(rows)
}

pub async fn update_list_vlastnictvi(pool: Pool, item: ListVlastnictvi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "UPDATE list_vlastnictvi SET katastralni_uzemi_id = $2, cislo_lv = $3, vlastnicky_hash = $4 WHERE id = $1",
        &[&item.id, &item.katastralni_uzemi_id, &item.cislo_lv, &item.vlastnicky_hash]
    ).await?;
    Ok(rows)
}

pub async fn delete_list_vlastnictvi(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM list_vlastnictvi WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- ParcelaRow ---
pub async fn get_parcela_row(pool: Pool) -> Result<Vec<ParcelaRow>> {
    let client = pool.get().await?;
    let rows = client.query("SELECT id, parcelni_cislo, cast_parcely, je_stavebni, vymera_metru_ctverecnich, ulice, cislo_popisne, katastralni_uzemi_id, bpej_id, list_vlastnictvi_id FROM parcela", &[]).await?;
    Ok(rows
        .iter()
        .map(|row| ParcelaRow {
            id: row.get(0),
            parcelni_cislo: row.get(1),
            cast_parcely: row.get(2),
            je_stavebni: row.get(3),
            vymera_metru_ctverecnich: row.get(4),
            ulice: row.get(5),
            cislo_popisne: row.get(6),
            katastralni_uzemi_id: row.get(7),
            bpej_id: row.get(8),
            list_vlastnictvi_id: row.get(9),
        })
        .collect())
}

pub async fn create_parcela_row(pool: Pool, item: NewParcelaRow) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO parcela (parcelni_cislo, cast_parcely, je_stavebni, vymera_metru_ctverecnich, ulice, cislo_popisne, katastralni_uzemi_id, bpej_id, list_vlastnictvi_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        &[&item.parcelni_cislo, &item.cast_parcely, &item.je_stavebni, &item.vymera_metru_ctverecnich, &item.ulice, &item.cislo_popisne, &item.katastralni_uzemi_id, &item.bpej_id, &item.list_vlastnictvi_id]
    ).await?;
    Ok(rows)
}

pub async fn update_parcela_row(pool: Pool, item: ParcelaRow) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "UPDATE parcela SET parcelni_cislo = $2, cast_parcely = $3, je_stavebni = $4, vymera_metru_ctverecnich = $5, ulice = $6, cislo_popisne = $7, katastralni_uzemi_id = $8, bpej_id = $9, list_vlastnictvi_id = $10 WHERE id = $1",
        &[&item.id, &item.parcelni_cislo, &item.cast_parcely, &item.je_stavebni, &item.vymera_metru_ctverecnich, &item.ulice, &item.cislo_popisne, &item.katastralni_uzemi_id, &item.bpej_id, &item.list_vlastnictvi_id]
    ).await?;
    Ok(rows)
}

pub async fn delete_parcela_row(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM parcela WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- Rizeni ---
pub async fn get_rizeni(pool: Pool) -> Result<Vec<Rizeni>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT id, rok, cislo_rizeni, typ_rizeni_id, predmet, poznamka FROM rizeni",
            &[],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|row| Rizeni {
            id: row.get(0),
            rok: row.get(1),
            cislo_rizeni: row.get(2),
            typ_rizeni_id: row.get(3),
            predmet: row.get(4),
            poznamka: row.get(5),
        })
        .collect())
}

pub async fn create_rizeni(pool: Pool, item: NewRizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO rizeni (rok, cislo_rizeni, typ_rizeni_id, predmet, poznamka) VALUES ($1, $2, $3, $4, $5)",
        &[&item.rok, &item.cislo_rizeni, &item.typ_rizeni_id, &item.predmet, &item.poznamka]
    ).await?;
    Ok(rows)
}

pub async fn update_rizeni(pool: Pool, item: Rizeni) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "UPDATE rizeni SET rok = $2, cislo_rizeni = $3, typ_rizeni_id = $4, predmet = $5, poznamka = $6 WHERE id = $1",
        &[&item.id, &item.rok, &item.cislo_rizeni, &item.typ_rizeni_id, &item.predmet, &item.poznamka]
    ).await?;
    Ok(rows)
}

pub async fn delete_rizeni(pool: Pool, id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute("DELETE FROM rizeni WHERE id = $1", &[&id])
        .await?;
    Ok(rows)
}

// --- Vlastnictvi ---
pub async fn get_vlastnictvi(pool: Pool) -> Result<Vec<Vlastnictvi>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT parcela_id, majitel_id, podil_setin FROM vlastnictvi",
            &[],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|row| Vlastnictvi {
            parcela_id: row.get(0),
            majitel_id: row.get(1),
            podil_setin: row.get(2),
        })
        .collect())
}

pub async fn create_vlastnictvi(pool: Pool, item: NewVlastnictvi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO vlastnictvi (parcela_id, majitel_id, podil_setin) VALUES ($1, $2, $3)",
            &[&item.parcela_id, &item.majitel_id, &item.podil_setin],
        )
        .await?;
    Ok(rows)
}

pub async fn update_vlastnictvi(pool: Pool, item: Vlastnictvi) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE vlastnictvi SET podil_setin = $3 WHERE parcela_id = $1 AND majitel_id = $2",
            &[&item.parcela_id, &item.majitel_id, &item.podil_setin],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_vlastnictvi(pool: Pool, parcela_id: i32, majitel_id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "DELETE FROM vlastnictvi WHERE parcela_id = $1 AND majitel_id = $2",
            &[&parcela_id, &majitel_id],
        )
        .await?;
    Ok(rows)
}

// --- BremenoParcelaParcela ---
pub async fn get_bremeno_parcela_parcela(pool: Pool) -> Result<Vec<BremenoParcelaParcela>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT parcela_id, parcela_povinna_id, popis, datum_zrizeni, datum_pravnich_ucinku FROM bremeno_parcela_parcela", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| BremenoParcelaParcela {
            parcela_id: row.get(0),
            parcela_povinna_id: row.get(1),
            popis: row.get(2),
            datum_zrizeni: row.get(3),
            datum_pravnich_ucinku: row.get(4),
        })
        .collect())
}

pub async fn create_bremeno_parcela_parcela(
    pool: Pool,
    item: NewBremenoParcelaParcela,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO bremeno_parcela_parcela (parcela_id, parcela_povinna_id, popis, datum_zrizeni, datum_pravnich_ucinku) VALUES ($1, $2, $3, $4, $5)",
        &[&item.parcela_id, &item.parcela_povinna_id, &item.popis, &item.datum_zrizeni, &item.datum_pravnich_ucinku]
    ).await?;
    Ok(rows)
}

pub async fn update_bremeno_parcela_parcela(
    pool: Pool,
    item: BremenoParcelaParcela,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "UPDATE bremeno_parcela_parcela SET popis = $3, datum_zrizeni = $4, datum_pravnich_ucinku = $5 WHERE parcela_id = $1 AND parcela_povinna_id = $2",
        &[&item.parcela_id, &item.parcela_povinna_id, &item.popis, &item.datum_zrizeni, &item.datum_pravnich_ucinku]
    ).await?;
    Ok(rows)
}

pub async fn delete_bremeno_parcela_parcela(
    pool: Pool,
    parcela_id: i32,
    parcela_povinna_id: i32,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "DELETE FROM bremeno_parcela_parcela WHERE parcela_id = $1 AND parcela_povinna_id = $2",
            &[&parcela_id, &parcela_povinna_id],
        )
        .await?;
    Ok(rows)
}

// --- BremenoParcelaMajitel ---
pub async fn get_bremeno_parcela_majitel(pool: Pool) -> Result<Vec<BremenoParcelaMajitel>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT parcela_id, majitel_povinny_id, popis, datum_zrizeni, datum_pravnich_ucinku FROM bremeno_parcela_majitel", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| BremenoParcelaMajitel {
            parcela_id: row.get(0),
            majitel_povinny_id: row.get(1),
            popis: row.get(2),
            datum_zrizeni: row.get(3),
            datum_pravnich_ucinku: row.get(4),
        })
        .collect())
}

pub async fn create_bremeno_parcela_majitel(
    pool: Pool,
    item: NewBremenoParcelaMajitel,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO bremeno_parcela_majitel (parcela_id, majitel_povinny_id, popis, datum_zrizeni, datum_pravnich_ucinku) VALUES ($1, $2, $3, $4, $5)",
        &[&item.parcela_id, &item.majitel_povinny_id, &item.popis, &item.datum_zrizeni, &item.datum_pravnich_ucinku]
    ).await?;
    Ok(rows)
}

pub async fn update_bremeno_parcela_majitel(
    pool: Pool,
    item: BremenoParcelaMajitel,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "UPDATE bremeno_parcela_majitel SET popis = $3, datum_zrizeni = $4, datum_pravnich_ucinku = $5 WHERE parcela_id = $1 AND majitel_povinny_id = $2",
        &[&item.parcela_id, &item.majitel_povinny_id, &item.popis, &item.datum_zrizeni, &item.datum_pravnich_ucinku]
    ).await?;
    Ok(rows)
}

pub async fn delete_bremeno_parcela_majitel(
    pool: Pool,
    parcela_id: i32,
    majitel_povinny_id: i32,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "DELETE FROM bremeno_parcela_majitel WHERE parcela_id = $1 AND majitel_povinny_id = $2",
            &[&parcela_id, &majitel_povinny_id],
        )
        .await?;
    Ok(rows)
}

// --- Plomba ---
pub async fn get_plomba(pool: Pool) -> Result<Vec<Plomba>> {
    let client = pool.get().await?;
    let rows = client
        .query("SELECT rizeni_id, parcela_id FROM plomba", &[])
        .await?;
    Ok(rows
        .iter()
        .map(|row| Plomba {
            rizeni_id: row.get(0),
            parcela_id: row.get(1),
        })
        .collect())
}

pub async fn create_plomba(pool: Pool, item: NewPlomba) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO plomba (rizeni_id, parcela_id) VALUES ($1, $2)",
            &[&item.rizeni_id, &item.parcela_id],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_plomba(pool: Pool, rizeni_id: i32, parcela_id: i32) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "DELETE FROM plomba WHERE rizeni_id = $1 AND parcela_id = $2",
            &[&rizeni_id, &parcela_id],
        )
        .await?;
    Ok(rows)
}

// --- RizeniOperaceRow ---
pub async fn get_rizeni_operace_row(pool: Pool) -> Result<Vec<RizeniOperaceRow>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT rizeni_id, typ_operace_id, datum FROM rizeni_operace",
            &[],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|row| RizeniOperaceRow {
            rizeni_id: row.get(0),
            typ_operace_id: row.get(1),
            datum: row.get(2),
        })
        .collect())
}

pub async fn create_rizeni_operace_row(pool: Pool, item: NewRizeniOperaceRow) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "INSERT INTO rizeni_operace (rizeni_id, typ_operace_id, datum) VALUES ($1, $2, $3)",
            &[&item.rizeni_id, &item.typ_operace_id, &item.datum],
        )
        .await?;
    Ok(rows)
}

pub async fn update_rizeni_operace_row(pool: Pool, item: RizeniOperaceRow) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "UPDATE rizeni_operace SET datum = $3 WHERE rizeni_id = $1 AND typ_operace_id = $2",
            &[&item.rizeni_id, &item.typ_operace_id, &item.datum],
        )
        .await?;
    Ok(rows)
}

pub async fn delete_rizeni_operace_row(
    pool: Pool,
    rizeni_id: i32,
    typ_operace_id: i32,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client
        .execute(
            "DELETE FROM rizeni_operace WHERE rizeni_id = $1 AND typ_operace_id = $2",
            &[&rizeni_id, &typ_operace_id],
        )
        .await?;
    Ok(rows)
}

// --- Ucast ---
pub async fn get_ucast(pool: Pool) -> Result<Vec<Ucast>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT rizeni_id, ucastnik_rizeni_id, typ_ucastnika_id FROM ucast",
            &[],
        )
        .await?;
    Ok(rows
        .iter()
        .map(|row| Ucast {
            rizeni_id: row.get(0),
            ucastnik_rizeni_id: row.get(1),
            typ_ucastnika_id: row.get(2),
        })
        .collect())
}

pub async fn create_ucast(pool: Pool, item: NewUcast) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute(
        "INSERT INTO ucast (rizeni_id, ucastnik_rizeni_id, typ_ucastnika_id) VALUES ($1, $2, $3)",
        &[&item.rizeni_id, &item.ucastnik_rizeni_id, &item.typ_ucastnika_id]
    ).await?;
    Ok(rows)
}

pub async fn delete_ucast(
    pool: Pool,
    rizeni_id: i32,
    ucastnik_rizeni_id: i32,
    typ_ucastnika_id: i32,
) -> Result<u64> {
    let client = pool.get().await?;
    let rows = client.execute("DELETE FROM ucast WHERE rizeni_id = $1 AND ucastnik_rizeni_id = $2 AND typ_ucastnika_id = $3", &[&rizeni_id, &ucastnik_rizeni_id, &typ_ucastnika_id]).await?;
    Ok(rows)
}







