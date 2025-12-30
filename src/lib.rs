#![feature(duration_millis_float)]

use anyhow::Result;
use deadpool_postgres::Pool;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use tokio_postgres::Statement;

pub mod endpoints;
pub mod templates;

pub struct DBState {
    pub prepared_statement: Statement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Majitel {
    pub jmeno: String,
    pub prijmeni: String,
    pub bydliste: String,
    pub podil_setin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parcela {
    pub parcelni_cislo: i64,
    pub je_stavebni: bool,
    pub ulice: Option<String>,
    pub cislo_popisne: Option<String>,
    pub nazev_ku: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParcelaB {
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
    pub je_stavebni_opravnena: bool,
    pub parcelni_cislo_opravnena: i64,
    pub cast_parcely_opravnena: i64,
    pub je_stavebni_povinna: bool,
    pub parcelni_cislo_povinna: i64,
    pub cast_parcely_povinna: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajitelB {
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
    pub je_stavebni_opravnena: bool,
    pub parcelni_cislo_opravnena: i64,
    pub cast_parcely_opravnena: i64,
    pub jmeno_povinny: String,
    pub prijmeni_povinny: String,
    pub titul_povinny: Option<String>,
    pub rodne_cislo_povinny: Option<String>,
    pub ico_povinny: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartC {
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
    pub je_stavebni_opravnena: bool,
    pub parcelni_cislo_opravnena: i64,
    pub cast_parcely_opravnena: i64,
    pub je_stavebni_povinna: bool,
    pub parcelni_cislo_povinna: i64,
    pub cast_parcely_povinna: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartD {
    pub je_stavebni: bool,
    pub parcelni_cislo: i64,
    pub cast_parcely: i64,
    pub nazev_katastralniho_uzemi: String,
    pub typ_rizeni_zkratka: String,
    pub cislo_rizeni: i64,
    pub rok_rizeni: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartF {
    pub je_stavebni: bool,
    pub parcelni_cislo: i64,
    pub cast_parcely: i64,
    pub hodnota: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindParcela {
    pub je_stavebni: bool,
    pub parcelni_cislo: i64,
    pub cast_parcely: i64,
    pub vymera_metru_ctverecnich: Option<rust_decimal::Decimal>,
    pub ulice: Option<String>,
    pub cislo_popisne: Option<String>,
    pub hodnota: Option<i64>,
    pub cislo_lv: i64,
}
pub async fn query_and_serialize_part_a(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
    let client = pool.get().await?;
    let rows = client.query(query, params).await?;

    let mut owners = Vec::new();
    for row in rows {
        let owner = Majitel {
            jmeno: row.try_get("jmeno")?,
            prijmeni: row.try_get("prijmeni")?,
            bydliste: row.try_get("bydliste")?,
            podil_setin: row.try_get::<_, i32>("podil_setin")? as i64,
        };
        owners.push(owner);
    }

    let json = serde_json::to_value(owners)?;
    Ok(json)
}

pub async fn query_and_serialize_part_b(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(parcels)?;
    Ok(json)
}

pub async fn query_and_serialize_part_b_parcela(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(parcels)?;
    Ok(json)
}

pub async fn query_and_serialize_part_b_majitel(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(items)?;
    Ok(json)
}

pub async fn query_and_serialize_part_c(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(items)?;
    Ok(json)
}

pub async fn query_and_serialize_part_d(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(items)?;
    Ok(json)
}

pub async fn query_and_serialize_part_f(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(items)?;
    Ok(json)
}

pub async fn query_and_serialize_parcela(
    pool: Pool,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<serde_json::Value> {
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

    let json = serde_json::to_value(items)?;
    Ok(json)
}
