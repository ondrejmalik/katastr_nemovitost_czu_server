import http from 'k6/http';
import { check, group } from 'k6';
import { Trend } from 'k6/metrics';

const BASE_URL = 'http://localhost:3000';
const PASSWORD = 'heslo';

// --- Configuration: Define START and END HTTP REQUESTS per second ---
const START_HTTP_REQ_PER_SEC = 5000;
const END_HTTP_REQ_PER_SEC = 5000;
const APPROX_REQ_PER_ITER = 75; // Estimated requests per full iteration cycle

// k6 controls "iterations", so we calculate iteration rates to hit the target RPS
const START_ITER_RATE = Math.ceil(START_HTTP_REQ_PER_SEC / APPROX_REQ_PER_ITER);
const END_ITER_RATE = Math.ceil(END_HTTP_REQ_PER_SEC / APPROX_REQ_PER_ITER);

export const options = {
  scenarios: {
    ramping_load: {
      executor: 'ramping-arrival-rate',
      startRate: START_ITER_RATE,
      timeUnit: '1s',
      preAllocatedVUs: 50,
      maxVUs: 300,
      stages: [
        { target: END_ITER_RATE, duration: '30s' },
      ],
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'], // <1% errors
    http_req_duration: ['p(95)<500'], // 95% of requests < 500ms
  },
};

// --- Custom Trends for Detailed Reporting ---
const t = {
    auth: new Trend('auth'),

    post_kraj: new Trend('post_kraj'),
    get_kraj: new Trend('get_kraj'),
    put_kraj: new Trend('put_kraj'),
    del_kraj: new Trend('del_kraj'),

    post_okres: new Trend('post_okres'),
    get_okres: new Trend('get_okres'),
    put_okres: new Trend('put_okres'),
    del_okres: new Trend('del_okres'),

    post_obec: new Trend('post_obec'),
    get_obec: new Trend('get_obec'),
    put_obec: new Trend('put_obec'),
    del_obec: new Trend('del_obec'),

    post_ku: new Trend('post_ku'),
    get_ku: new Trend('get_ku'),
    put_ku: new Trend('put_ku'),
    del_ku: new Trend('del_ku'),

    post_bpej: new Trend('post_bpej'),
    get_bpej: new Trend('get_bpej'),
    put_bpej: new Trend('put_bpej'),
    del_bpej: new Trend('del_bpej'),

    post_typ_rizeni: new Trend('post_typ_rizeni'),
    get_typ_rizeni: new Trend('get_typ_rizeni'),
    put_typ_rizeni: new Trend('put_typ_rizeni'),
    del_typ_rizeni: new Trend('del_typ_rizeni'),

    post_typ_operace: new Trend('post_typ_operace'),
    get_typ_operace: new Trend('get_typ_operace'),
    put_typ_operace: new Trend('put_typ_operace'),
    del_typ_operace: new Trend('del_typ_operace'),

    post_typ_ucastnika: new Trend('post_typ_ucastnika'),
    get_typ_ucastnika: new Trend('get_typ_ucastnika'),
    put_typ_ucastnika: new Trend('put_typ_ucastnika'),
    del_typ_ucastnika: new Trend('del_typ_ucastnika'),

    post_ucastnik: new Trend('post_ucastnik'),
    get_ucastnik: new Trend('get_ucastnik'),
    put_ucastnik: new Trend('put_ucastnik'),
    del_ucastnik: new Trend('del_ucastnik'),

    post_lv: new Trend('post_lv'),
    get_lv: new Trend('get_lv'), // simple get list
    get_lv_detail: new Trend('get_lv_detail'), // complex get
    put_lv: new Trend('put_lv'),
    del_lv: new Trend('del_lv'),

    post_parcela: new Trend('post_parcela'),
    get_parcela: new Trend('get_parcela'), // simple get list
    get_parcela_detail: new Trend('get_parcela_detail'), // complex get
    put_parcela: new Trend('put_parcela'),
    del_parcela: new Trend('del_parcela'),

    post_majitel: new Trend('post_majitel'),
    get_majitel: new Trend('get_majitel'),
    put_majitel: new Trend('put_majitel'),
    del_majitel: new Trend('del_majitel'),

    post_rizeni: new Trend('post_rizeni'),
    get_rizeni: new Trend('get_rizeni'),
    get_rizeni_detail: new Trend('get_rizeni_detail'),
    put_rizeni: new Trend('put_rizeni'),
    del_rizeni: new Trend('del_rizeni'),

    post_vlastnictvi: new Trend('post_vlastnictvi'),
    put_vlastnictvi: new Trend('put_vlastnictvi'),
    del_vlastnictvi: new Trend('del_vlastnictvi'),

    post_bremeno_pp: new Trend('post_bremeno_pp'),
    put_bremeno_pp: new Trend('put_bremeno_pp'),
    del_bremeno_pp: new Trend('del_bremeno_pp'),

    post_bremeno_pm: new Trend('post_bremeno_pm'),
    put_bremeno_pm: new Trend('put_bremeno_pm'),
    del_bremeno_pm: new Trend('del_bremeno_pm'),

    post_rizeni_operace: new Trend('post_rizeni_operace'),
    put_rizeni_operace: new Trend('put_rizeni_operace'),
    del_rizeni_operace: new Trend('del_rizeni_operace'),

    post_ucast: new Trend('post_ucast'),
    del_ucast: new Trend('del_ucast'),

    post_plomba: new Trend('post_plomba'),
    del_plomba: new Trend('del_plomba'),
};

// Helper to execute request and track trend
function exec(method, url, body, params, trend) {
    let res;
    if (method === 'GET') res = http.get(url, params);
    else if (method === 'POST') res = http.post(url, body, params);
    else if (method === 'PUT') res = http.put(url, body, params);
    else if (method === 'DELETE') res = http.del(url, body, params);

    if (res.status >= 400) {
        console.error(`Request Failed: ${method} ${url} - Status: ${res.status} - Body: ${res.body ? res.body.substring(0, 100) : 'empty'}`);
    }

    if (trend) trend.add(res.timings.duration);
    return res;
}
// Helper to find ID safely
function findId(response, field, value) {
  try {
      const items = response.json();
      if (!Array.isArray(items)) return null;
      const item = items.find(i => i[field] === value);
      return item ? item.id : null;
  } catch (e) {
      return null;
  }
}

export function setup() {
  const res = http.get(`${BASE_URL}/auth?password=${PASSWORD}`);
  if (res.status !== 200) {
      throw new Error(`Auth failed in setup: ${res.status} ${res.body}`);
  }
  const sessionCookie = res.cookies['katastr_session'][0].value;
  return { sessionCookie };
}

export default function (data) {
  const uniqueSuffix = `${Date.now()}_${Math.floor(Math.random() * 1000000)}`;
  const jar = http.cookieJar();

  if (!jar.cookiesForURL(BASE_URL).katastr_session) {
      jar.set(BASE_URL, 'katastr_session', data.sessionCookie);
  }

  const jsonParams = { headers: { 'Content-Type': 'application/json' } };

  let ids = {};

  // 2. Hierarchy
  group('Hierarchy', function () {
    // Kraj
    exec('POST', `${BASE_URL}/kraj`, JSON.stringify({ nazev: `K6_Kraj_${uniqueSuffix}` }), jsonParams, t.post_kraj);
    let res = exec('GET', `${BASE_URL}/kraj`, null, null, t.get_kraj);
    ids.kraj = findId(res, 'nazev', `K6_Kraj_${uniqueSuffix}`);

    if(ids.kraj) exec('PUT', `${BASE_URL}/kraj`, JSON.stringify({ id: ids.kraj, nazev: `K6_Kraj_${uniqueSuffix}_U` }), jsonParams, t.put_kraj);

    // Okres
    if(ids.kraj) {
        exec('POST', `${BASE_URL}/okres`, JSON.stringify({ kraj_id: ids.kraj, nazev: `K6_Okres_${uniqueSuffix}` }), jsonParams, t.post_okres);
        res = exec('GET', `${BASE_URL}/okres`, null, null, t.get_okres);
        ids.okres = findId(res, 'nazev', `K6_Okres_${uniqueSuffix}`);
        if(ids.okres) exec('PUT', `${BASE_URL}/okres`, JSON.stringify({ id: ids.okres, kraj_id: ids.kraj, nazev: `K6_Okres_${uniqueSuffix}_U` }), jsonParams, t.put_okres);
    }

    // Obec
    if(ids.okres) {
        exec('POST', `${BASE_URL}/obec`, JSON.stringify({ okres_id: ids.okres, nazev: `K6_Obec_${uniqueSuffix}` }), jsonParams, t.post_obec);
        res = exec('GET', `${BASE_URL}/obec`, null, null, t.get_obec);
        ids.obec = findId(res, 'nazev', `K6_Obec_${uniqueSuffix}`);
        if(ids.obec) exec('PUT', `${BASE_URL}/obec`, JSON.stringify({ id: ids.obec, okres_id: ids.okres, nazev: `K6_Obec_${uniqueSuffix}_U` }), jsonParams, t.put_obec);
    }

    // KU
    if(ids.obec) {
        exec('POST', `${BASE_URL}/katastralni_uzemi`, JSON.stringify({ obec_id: ids.obec, nazev: `K6_KU_${uniqueSuffix}` }), jsonParams, t.post_ku);
        res = exec('GET', `${BASE_URL}/katastralni_uzemi`, null, null, t.get_ku);
        ids.ku = findId(res, 'nazev', `K6_KU_${uniqueSuffix}`);
        if(ids.ku) exec('PUT', `${BASE_URL}/katastralni_uzemi`, JSON.stringify({ id: ids.ku, obec_id: ids.obec, nazev: `K6_KU_${uniqueSuffix}_U` }), jsonParams, t.put_ku);
    }
  });

  // 3. Metadata
  group('Metadata', function () {
    let bpejVal = Math.floor(Math.random() * 2147483647);
    exec('POST', `${BASE_URL}/bpej`, JSON.stringify({ hodnota: bpejVal }), jsonParams, t.post_bpej);
    ids.bpej = findId(exec('GET', `${BASE_URL}/bpej`, null, null, t.get_bpej), 'hodnota', bpejVal);
    if(ids.bpej) exec('PUT', `${BASE_URL}/bpej`, JSON.stringify({ id: ids.bpej, hodnota: bpejVal + 1 }), jsonParams, t.put_bpej);

    exec('POST', `${BASE_URL}/typ_rizeni`, JSON.stringify({ nazev: `K6_TR_${uniqueSuffix}`, zkratka: `K6_${uniqueSuffix}` }), jsonParams, t.post_typ_rizeni);
    ids.typ_rizeni = findId(exec('GET', `${BASE_URL}/typ_rizeni`, null, null, t.get_typ_rizeni), 'nazev', `K6_TR_${uniqueSuffix}`);
    if(ids.typ_rizeni) exec('PUT', `${BASE_URL}/typ_rizeni`, JSON.stringify({ id: ids.typ_rizeni, nazev: `K6_TR_${uniqueSuffix}_U`, zkratka: `K6_${uniqueSuffix}` }), jsonParams, t.put_typ_rizeni);

    exec('POST', `${BASE_URL}/typ_operace`, JSON.stringify({ popis: `K6_TO_${uniqueSuffix}` }), jsonParams, t.post_typ_operace);
    ids.typ_operace = findId(exec('GET', `${BASE_URL}/typ_operace`, null, null, t.get_typ_operace), 'popis', `K6_TO_${uniqueSuffix}`);
    if(ids.typ_operace) exec('PUT', `${BASE_URL}/typ_operace`, JSON.stringify({ id: ids.typ_operace, popis: `K6_TO_${uniqueSuffix}_U` }), jsonParams, t.put_typ_operace);

    exec('POST', `${BASE_URL}/typ_ucastnika`, JSON.stringify({ nazev: `K6_TU_${uniqueSuffix}` }), jsonParams, t.post_typ_ucastnika);
    ids.typ_ucastnika = findId(exec('GET', `${BASE_URL}/typ_ucastnika`, null, null, t.get_typ_ucastnika), 'nazev', `K6_TU_${uniqueSuffix}`);
    if(ids.typ_ucastnika) exec('PUT', `${BASE_URL}/typ_ucastnika`, JSON.stringify({ id: ids.typ_ucastnika, nazev: `K6_TU_${uniqueSuffix}_U` }), jsonParams, t.put_typ_ucastnika);

    exec('POST', `${BASE_URL}/ucastnik_rizeni`, JSON.stringify({ jmeno: `K6_User_${uniqueSuffix}` }), jsonParams, t.post_ucastnik);
    ids.ucastnik_rizeni = findId(exec('GET', `${BASE_URL}/ucastnik_rizeni`, null, null, t.get_ucastnik), 'jmeno', `K6_User_${uniqueSuffix}`);
    if(ids.ucastnik_rizeni) exec('PUT', `${BASE_URL}/ucastnik_rizeni`, JSON.stringify({ id: ids.ucastnik_rizeni, jmeno: `K6_User_${uniqueSuffix}_U` }), jsonParams, t.put_ucastnik);
  });

  // 4. Entities
  if(ids.ku) {
    group('Entities', function () {
      let cisloLv = Math.floor(Math.random() * 2147483647);
      exec('POST', `${BASE_URL}/list_vlastnictvi`, JSON.stringify({ katastralni_uzemi_id: ids.ku, cislo_lv: cisloLv, vlastnicky_hash: `h_${uniqueSuffix}` }), jsonParams, t.post_lv);
      let lvRes = exec('GET', `${BASE_URL}/list_vlastnictvi`, null, null, t.get_lv);
      try { ids.lv = lvRes.json().find(i => i.cislo_lv === cisloLv).id; } catch(e) {}
      ids.cislo_lv = cisloLv;
      if(ids.lv) exec('PUT', `${BASE_URL}/list_vlastnictvi`, JSON.stringify({ id: ids.lv, katastralni_uzemi_id: ids.ku, cislo_lv: cisloLv, vlastnicky_hash: `h_${uniqueSuffix}_U` }), jsonParams, t.put_lv);

      if(ids.lv && ids.bpej) {
        let pc1 = Math.floor(Math.random() * 1000000);
        exec('POST', `${BASE_URL}/parcela_row`, JSON.stringify({ parcelni_cislo: pc1, cast_parcely: 1, je_stavebni: false, vymera_metru_ctverecnich: 100, ulice: "K6", cislo_popisne: "1", katastralni_uzemi_id: ids.ku, bpej_id: ids.bpej, list_vlastnictvi_id: ids.lv }), jsonParams, t.post_parcela);
        let pRes = exec('GET', `${BASE_URL}/parcela_row`, null, null, t.get_parcela);
        try { ids.parcela = pRes.json().find(i => i.parcelni_cislo === pc1).id; } catch(e) {}
        ids.parcelni_cislo = pc1;
        if(ids.parcela) exec('PUT', `${BASE_URL}/parcela_row`, JSON.stringify({ id: ids.parcela, parcelni_cislo: pc1, cast_parcely: 1, je_stavebni: false, vymera_metru_ctverecnich: 101, ulice: "K6", cislo_popisne: "1", katastralni_uzemi_id: ids.ku, bpej_id: ids.bpej, list_vlastnictvi_id: ids.lv }), jsonParams, t.put_parcela);

        let pc2 = pc1 + 1;
        exec('POST', `${BASE_URL}/parcela_row`, JSON.stringify({ parcelni_cislo: pc2, cast_parcely: 1, je_stavebni: false, vymera_metru_ctverecnich: 200, ulice: "K6", cislo_popisne: "2", katastralni_uzemi_id: ids.ku, bpej_id: ids.bpej, list_vlastnictvi_id: ids.lv }), jsonParams, t.post_parcela);
        try { ids.parcela2 = exec('GET', `${BASE_URL}/parcela_row`, null, null, null).json().find(i => i.parcelni_cislo === pc2).id; } catch(e) {}
      }

      exec('POST', `${BASE_URL}/majitel`, JSON.stringify({ jmeno: "K6", prijmeni: `M_${uniqueSuffix}`, titul: "Mr.", bydliste: "City", rodne_cislo: `RC_${uniqueSuffix}`, ico: `ICO_${uniqueSuffix}` }), jsonParams, t.post_majitel);
      ids.majitel = findId(exec('GET', `${BASE_URL}/majitel`, null, null, t.get_majitel), 'prijmeni', `M_${uniqueSuffix}`);
      if(ids.majitel) exec('PUT', `${BASE_URL}/majitel`, JSON.stringify({ id: ids.majitel, jmeno: "K6U", prijmeni: `M_${uniqueSuffix}`, titul: "Mr.", bydliste: "City", rodne_cislo: `RC_${uniqueSuffix}`, ico: `ICO_${uniqueSuffix}` }), jsonParams, t.put_majitel);

      if(ids.typ_rizeni) {
        exec('POST', `${BASE_URL}/rizeni`, JSON.stringify({ rok: 2026, cislo_rizeni: Math.floor(Math.random() * 1000000), typ_rizeni_id: ids.typ_rizeni, predmet: `P_${uniqueSuffix}`, poznamka: "N" }), jsonParams, t.post_rizeni);
        ids.rizeni = findId(exec('GET', `${BASE_URL}/rizeni`, null, null, t.get_rizeni), 'predmet', `P_${uniqueSuffix}`);
        if(ids.rizeni) exec('PUT', `${BASE_URL}/rizeni`, JSON.stringify({ id: ids.rizeni, rok: 2026, cislo_rizeni: Math.floor(Math.random() * 1000000), typ_rizeni_id: ids.typ_rizeni, predmet: `P_${uniqueSuffix}`, poznamka: "NU" }), jsonParams, t.put_rizeni);
      }
    });

    // 5. Setup Links (only if we have IDs)
    if(ids.parcela && ids.majitel && ids.rizeni && ids.ucastnik_rizeni && ids.typ_ucastnika && ids.typ_operace) {
      group('Links', function () {
        exec('POST', `${BASE_URL}/vlastnictvi`, JSON.stringify({ parcela_id: ids.parcela, majitel_id: ids.majitel, podil_setin: 100 }), jsonParams, t.post_vlastnictvi);
        exec('PUT', `${BASE_URL}/vlastnictvi`, JSON.stringify({ parcela_id: ids.parcela, majitel_id: ids.majitel, podil_setin: 50 }), jsonParams, t.put_vlastnictvi);

        if(ids.parcela2) {
          exec('POST', `${BASE_URL}/bremeno_parcela_parcela`, JSON.stringify({ parcela_id: ids.parcela, parcela_povinna_id: ids.parcela2, popis: "B", datum_zrizeni: "2026-01-01", datum_pravnich_ucinku: "2026-01-01" }), jsonParams, t.post_bremeno_pp);
          exec('PUT', `${BASE_URL}/bremeno_parcela_parcela`, JSON.stringify({ parcela_id: ids.parcela, parcela_povinna_id: ids.parcela2, popis: "BU", datum_zrizeni: "2026-01-01", datum_pravnich_ucinku: "2026-01-01" }), jsonParams, t.put_bremeno_pp);
        }

        exec('POST', `${BASE_URL}/bremeno_parcela_majitel`, JSON.stringify({ parcela_id: ids.parcela, majitel_povinny_id: ids.majitel, popis: "B", datum_zrizeni: "2026-01-01", datum_pravnich_ucinku: "2026-01-01" }), jsonParams, t.post_bremeno_pm);
        exec('PUT', `${BASE_URL}/bremeno_parcela_majitel`, JSON.stringify({ parcela_id: ids.parcela, majitel_povinny_id: ids.majitel, popis: "BU", datum_zrizeni: "2026-01-01", datum_pravnich_ucinku: "2026-01-01" }), jsonParams, t.put_bremeno_pm);

        exec('POST', `${BASE_URL}/rizeni_operace`, JSON.stringify({ rizeni_id: ids.rizeni, typ_operace_id: ids.typ_operace, datum: "2026-01-01" }), jsonParams, t.post_rizeni_operace);
        exec('PUT', `${BASE_URL}/rizeni_operace`, JSON.stringify({ rizeni_id: ids.rizeni, typ_operace_id: ids.typ_operace, datum: "2026-02-01" }), jsonParams, t.put_rizeni_operace);

        exec('POST', `${BASE_URL}/ucast`, JSON.stringify({ rizeni_id: ids.rizeni, ucastnik_rizeni_id: ids.ucastnik_rizeni, typ_ucastnika_id: ids.typ_ucastnika }), jsonParams, t.post_ucast);
        exec('POST', `${BASE_URL}/plomba`, JSON.stringify({ rizeni_id: ids.rizeni, parcela_id: ids.parcela }), jsonParams, t.post_plomba);
      });
    }

    // 6. Complex GETs
    if(ids.lv) exec('GET', `${BASE_URL}/lv?katastralni_uzemi=Město+Brno&cislo_lv=1`, null, null, t.get_lv_detail);
    if(ids.parcela) exec('GET', 'http://localhost:3000/parcela?katastralni_uzemi=Město+Brno&je_stavebni=true&parcelni_cislo=1&cast_parcely=1', null, null, t.get_parcela_detail);
    if(ids.rizeni) exec('GET', `${BASE_URL}/spravni_rizeni?id=${ids.rizeni}`, null, null, t.get_rizeni_detail);
  }

  // 7. Cleanup (only if created)
  group('Cleanup', function () {
    if(ids.rizeni) {
        if(ids.parcela) exec('DELETE', `${BASE_URL}/plomba?rizeni_id=${ids.rizeni}&parcela_id=${ids.parcela}`, null, null, t.del_plomba);
        if(ids.ucastnik_rizeni) exec('DELETE', `${BASE_URL}/ucast?rizeni_id=${ids.rizeni}&ucastnik_rizeni_id=${ids.ucastnik_rizeni}&typ_ucastnika_id=${ids.typ_ucastnika}`, null, null, t.del_ucast);
        if(ids.typ_operace) exec('DELETE', `${BASE_URL}/rizeni_operace?rizeni_id=${ids.rizeni}&typ_operace_id=${ids.typ_operace}`, null, null, t.del_rizeni_operace);
    }

    if(ids.parcela && ids.majitel) {
        exec('DELETE', `${BASE_URL}/bremeno_parcela_majitel?parcela_id=${ids.parcela}&majitel_povinny_id=${ids.majitel}`, null, null, t.del_bremeno_pm);
        exec('DELETE', `${BASE_URL}/vlastnictvi?parcela_id=${ids.parcela}&majitel_id=${ids.majitel}`, null, null, t.del_vlastnictvi);
    }
    if(ids.parcela && ids.parcela2) exec('DELETE', `${BASE_URL}/bremeno_parcela_parcela?parcela_id=${ids.parcela}&parcela_povinna_id=${ids.parcela2}`, null, null, t.del_bremeno_pp);

    if(ids.rizeni) exec('DELETE', `${BASE_URL}/rizeni?id=${ids.rizeni}`, null, null, t.del_rizeni);
    if(ids.ucastnik_rizeni) exec('DELETE', `${BASE_URL}/ucastnik_rizeni?id=${ids.ucastnik_rizeni}`, null, null, t.del_ucastnik);
    if(ids.typ_ucastnika) exec('DELETE', `${BASE_URL}/typ_ucastnika?id=${ids.typ_ucastnika}`, null, null, t.del_typ_ucastnika);
    if(ids.typ_operace) exec('DELETE', `${BASE_URL}/typ_operace?id=${ids.typ_operace}`, null, null, t.del_typ_operace);
    if(ids.typ_rizeni) exec('DELETE', `${BASE_URL}/typ_rizeni?id=${ids.typ_rizeni}`, null, null, t.del_typ_rizeni);
    if(ids.majitel) exec('DELETE', `${BASE_URL}/majitel?id=${ids.majitel}`, null, null, t.del_majitel);
    if(ids.parcela) exec('DELETE', `${BASE_URL}/parcela_row?id=${ids.parcela}`, null, null, t.del_parcela);
    if(ids.parcela2) exec('DELETE', `${BASE_URL}/parcela_row?id=${ids.parcela2}`, null, null, t.del_parcela);
    if(ids.lv) exec('DELETE', `${BASE_URL}/list_vlastnictvi?id=${ids.lv}`, null, null, t.del_lv);
    if(ids.bpej) exec('DELETE', `${BASE_URL}/bpej?id=${ids.bpej}`, null, null, t.del_bpej);
    if(ids.ku) exec('DELETE', `${BASE_URL}/katastralni_uzemi?id=${ids.ku}`, null, null, t.del_ku);
    if(ids.obec) exec('DELETE', `${BASE_URL}/obec?id=${ids.obec}`, null, null, t.del_obec);
    if(ids.okres) exec('DELETE', `${BASE_URL}/okres?id=${ids.okres}`, null, null, t.del_okres);
    if(ids.kraj) exec('DELETE', `${BASE_URL}/kraj?id=${ids.kraj}`, null, null, t.del_kraj);
  });
}
