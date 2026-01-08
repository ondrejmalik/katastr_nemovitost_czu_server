use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(Clone)]
pub struct AppState {
    pub password: String,
    pub no_print: bool,
    pub sessions: Arc<RwLock<HashMap<String, Instant>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajitelPartA {
    pub jmeno: String,
    pub prijmeni: String,
    pub bydliste: String,
    pub podil_setin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Majitel {
    pub id: i32,
    pub jmeno: String,
    pub prijmeni: String,
    pub titul: Option<String>,
    pub bydliste: Option<String>,
    pub rodne_cislo: Option<String>,
    pub ico: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMajitel {
    pub jmeno: String,
    pub prijmeni: String,
    pub titul: Option<String>,
    pub bydliste: Option<String>,
    pub rodne_cislo: Option<String>,
    pub ico: Option<String>,
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
    pub vymera_metru_ctverecnich: Option<Decimal>,
    pub ulice: Option<String>,
    pub cislo_popisne: Option<String>,
    pub hodnota: Option<i64>,
    pub cislo_lv: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RizeniPredmetPoznamka {
    pub predmet: String,
    pub poznamka: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RizeniUcastnik {
    pub typ_ucastnika: String,
    pub ucastnik_jmeno: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RizeniOperace {
    pub operace_popis: String,
    pub operace_datum: Option<chrono::NaiveDate>,
}

// --- Kraj ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kraj {
    pub id: i32,
    pub nazev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKraj {
    pub nazev: String,
}

// --- Okres ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Okres {
    pub id: i32,
    pub kraj_id: i32,
    pub nazev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOkres {
    pub kraj_id: i32,
    pub nazev: String,
}

// --- Obec ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obec {
    pub id: i32,
    pub okres_id: i32,
    pub nazev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewObec {
    pub okres_id: i32,
    pub nazev: String,
}

// --- KatastralniUzemi ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KatastralniUzemi {
    pub id: i32,
    pub obec_id: i32,
    pub nazev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKatastralniUzemi {
    pub obec_id: i32,
    pub nazev: String,
}

// --- Bpej ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bpej {
    pub id: i32,
    pub hodnota: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBpej {
    pub hodnota: i32,
}

// --- TypRizeni ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypRizeni {
    pub id: i32,
    pub nazev: String,
    pub zkratka: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTypRizeni {
    pub nazev: String,
    pub zkratka: String,
}

// --- TypOperace ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypOperace {
    pub id: i32,
    pub popis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTypOperace {
    pub popis: String,
}

// --- TypUcastnika ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypUcastnika {
    pub id: i32,
    pub nazev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTypUcastnika {
    pub nazev: String,
}

// --- UcastnikRizeni ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcastnikRizeni {
    pub id: i32,
    pub jmeno: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUcastnikRizeni {
    pub jmeno: String,
}

// --- ListVlastnictvi ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListVlastnictvi {
    pub id: i32,
    pub katastralni_uzemi_id: i32,
    pub cislo_lv: i32,
    pub vlastnicky_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewListVlastnictvi {
    pub katastralni_uzemi_id: i32,
    pub cislo_lv: i32,
    pub vlastnicky_hash: Option<String>,
}

// --- ParcelaRow ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParcelaRow {
    pub id: i32,
    pub parcelni_cislo: i32,
    pub cast_parcely: i32,
    pub je_stavebni: bool,
    pub vymera_metru_ctverecnich: Decimal,
    pub ulice: Option<String>,
    pub cislo_popisne: Option<String>,
    pub katastralni_uzemi_id: i32,
    pub bpej_id: Option<i32>,
    pub list_vlastnictvi_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewParcelaRow {
    pub parcelni_cislo: i32,
    pub cast_parcely: i32,
    pub je_stavebni: bool,
    pub vymera_metru_ctverecnich: Decimal,
    pub ulice: Option<String>,
    pub cislo_popisne: Option<String>,
    pub katastralni_uzemi_id: i32,
    pub bpej_id: Option<i32>,
    pub list_vlastnictvi_id: i32,
}

// --- Rizeni ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rizeni {
    pub id: i32,
    pub rok: i32,
    pub cislo_rizeni: i32,
    pub typ_rizeni_id: i32,
    pub predmet: String,
    pub poznamka: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRizeni {
    pub rok: i32,
    pub cislo_rizeni: i32,
    pub typ_rizeni_id: i32,
    pub predmet: String,
    pub poznamka: Option<String>,
}

// --- Vlastnictvi ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vlastnictvi {
    pub parcela_id: i32,
    pub majitel_id: i32,
    pub podil_setin: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVlastnictvi {
    pub parcela_id: i32,
    pub majitel_id: i32,
    pub podil_setin: i32,
}

// --- BremenoParcelaParcela ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BremenoParcelaParcela {
    pub parcela_id: i32,
    pub parcela_povinna_id: i32,
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBremenoParcelaParcela {
    pub parcela_id: i32,
    pub parcela_povinna_id: i32,
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
}

// --- BremenoParcelaMajitel ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BremenoParcelaMajitel {
    pub parcela_id: i32,
    pub majitel_povinny_id: i32,
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewBremenoParcelaMajitel {
    pub parcela_id: i32,
    pub majitel_povinny_id: i32,
    pub popis: String,
    pub datum_zrizeni: chrono::NaiveDate,
    pub datum_pravnich_ucinku: chrono::NaiveDate,
}

// --- Plomba ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plomba {
    pub rizeni_id: i32,
    pub parcela_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPlomba {
    pub rizeni_id: i32,
    pub parcela_id: i32,
}

// --- RizeniOperaceRow ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RizeniOperaceRow {
    pub rizeni_id: i32,
    pub typ_operace_id: i32,
    pub datum: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRizeniOperaceRow {
    pub rizeni_id: i32,
    pub typ_operace_id: i32,
    pub datum: chrono::NaiveDate,
}

// --- Ucast ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ucast {
    pub rizeni_id: i32,
    pub ucastnik_rizeni_id: i32,
    pub typ_ucastnika_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUcast {
    pub rizeni_id: i32,
    pub ucastnik_rizeni_id: i32,
    pub typ_ucastnika_id: i32,
}
