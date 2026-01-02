import subprocess
import json
import time
import random

BASE_URL = "http://localhost:3000"
SUFFIX = f"_TEST_{random.randint(1000, 9999)}"

def run_curl(method, endpoint, data=None, params=None):
    url = f"{BASE_URL}{endpoint}"
    if params:
        query_string = "&".join([f"{k}={v}" for k, v in params.items()])
        url += f"?{query_string}"

    cmd = ["curl", "-s", "-X", method, url]
    
    if data:
        cmd.extend(["-H", "Content-Type: application/json"])
        cmd.extend(["-d", json.dumps(data)])
        
    print(f"CMD: {' '.join(cmd)}")
    result = subprocess.run(cmd, capture_output=True)
    
    if result.returncode != 0:
        print(f"Error: {result.stderr.decode('utf-8', errors='replace')}")
        return None
        
    if not result.stdout:
        return None

    stdout_str = result.stdout.decode('utf-8', errors='replace')
    try:
        return json.loads(stdout_str)
    except json.JSONDecodeError:
        print(f"Raw output: {stdout_str}")
        return stdout_str

def get_id(items, key, value, id_field="id"):
    if not isinstance(items, list):
        print(f"Expected list, got {type(items)}")
        return None
    for item in items:
        if item.get(key) == value:
            return item.get(id_field)
    return None

ids = {}

# 1. Kraj
print("\n--- Testing Kraj ---")
kraj_name = f"Kraj{SUFFIX}"
run_curl("POST", "/kraj", {"nazev": kraj_name})
krajs = run_curl("GET", "/kraj")
ids["kraj"] = get_id(krajs, "nazev", kraj_name)
print(f"Created Kraj ID: {ids['kraj']}")
if ids["kraj"]:
    run_curl("PUT", "/kraj", {"id": ids["kraj"], "nazev": f"{kraj_name}_UPD"})

# 2. Okres
print("\n--- Testing Okres ---")
if ids.get("kraj"):
    okres_name = f"Okres{SUFFIX}"
    run_curl("POST", "/okres", {"kraj_id": ids["kraj"], "nazev": okres_name})
    okresy = run_curl("GET", "/okres")
    ids["okres"] = get_id(okresy, "nazev", okres_name)
    print(f"Created Okres ID: {ids['okres']}")

# 3. Obec
print("\n--- Testing Obec ---")
if ids.get("okres"):
    obec_name = f"Obec{SUFFIX}"
    run_curl("POST", "/obec", {"okres_id": ids["okres"], "nazev": obec_name})
    obce = run_curl("GET", "/obec")
    ids["obec"] = get_id(obce, "nazev", obec_name)
    print(f"Created Obec ID: {ids['obec']}")

# 4. KatastralniUzemi
print("\n--- Testing KatastralniUzemi ---")
if ids.get("obec"):
    ku_name = f"KU{SUFFIX}"
    run_curl("POST", "/katastralni_uzemi", {"obec_id": ids["obec"], "nazev": ku_name})
    kus = run_curl("GET", "/katastralni_uzemi")
    ids["ku"] = get_id(kus, "nazev", ku_name)
    print(f"Created KU ID: {ids['ku']}")

# 5. Bpej
print("\n--- Testing Bpej ---")
bpej_val = random.randint(10000, 99999)
run_curl("POST", "/bpej", {"hodnota": bpej_val})
bpejs = run_curl("GET", "/bpej")
ids["bpej"] = get_id(bpejs, "hodnota", bpej_val)
print(f"Created Bpej ID: {ids['bpej']}")

# 6. TypRizeni
print("\n--- Testing TypRizeni ---")
tr_name = f"TR{SUFFIX}"
run_curl("POST", "/typ_rizeni", {"nazev": tr_name, "zkratka": "TR"})
trs = run_curl("GET", "/typ_rizeni")
ids["typ_rizeni"] = get_id(trs, "nazev", tr_name)
print(f"Created TypRizeni ID: {ids['typ_rizeni']}")

# 7. TypOperace
print("\n--- Testing TypOperace ---")
to_name = f"TO{SUFFIX}"
run_curl("POST", "/typ_operace", {"popis": to_name})
tos = run_curl("GET", "/typ_operace")
ids["typ_operace"] = get_id(tos, "popis", to_name)
print(f"Created TypOperace ID: {ids['typ_operace']}")

# 8. TypUcastnika
print("\n--- Testing TypUcastnika ---")
tu_name = f"TU{SUFFIX}"
run_curl("POST", "/typ_ucastnika", {"nazev": tu_name})
tus = run_curl("GET", "/typ_ucastnika")
ids["typ_ucastnika"] = get_id(tus, "nazev", tu_name)
print(f"Created TypUcastnika ID: {ids['typ_ucastnika']}")

# 9. UcastnikRizeni
print("\n--- Testing UcastnikRizeni ---")
ur_name = f"UR{SUFFIX}"
run_curl("POST", "/ucastnik_rizeni", {"jmeno": ur_name})
urs = run_curl("GET", "/ucastnik_rizeni")
ids["ucastnik_rizeni"] = get_id(urs, "jmeno", ur_name)
print(f"Created UcastnikRizeni ID: {ids['ucastnik_rizeni']}")

# 10. Majitel
print("\n--- Testing Majitel ---")
majitel_jmeno = f"Jan{SUFFIX}"
rc_suffix = random.randint(1000, 9999)
run_curl("POST", "/majitel", {
    "jmeno": majitel_jmeno,
    "prijmeni": "Novak",
    "titul": None,
    "bydliste": "Praha",
    "rodne_cislo": f"123456/{rc_suffix}",
    "ico": None
})
majitele = run_curl("GET", "/majitel")
ids["majitel"] = get_id(majitele, "jmeno", majitel_jmeno)
print(f"Created Majitel ID: {ids['majitel']}")

# 11. ListVlastnictvi
print("\n--- Testing ListVlastnictvi ---")
if ids.get("ku"):
    lv_cislo = random.randint(100, 999)
    run_curl("POST", "/list_vlastnictvi", {
        "katastralni_uzemi_id": ids["ku"],
        "cislo_lv": lv_cislo,
        "vlastnicky_hash": None
    })
    lvs = run_curl("GET", "/list_vlastnictvi")
    # Need to filter by KU and cislo_lv
    if isinstance(lvs, list):
        for item in lvs:
            if item["katastralni_uzemi_id"] == ids["ku"] and item["cislo_lv"] == lv_cislo:
                ids["lv"] = item["id"]
                break
    print(f"Created LV ID: {ids['lv']}")

# 12. ParcelaRow
print("\n--- Testing ParcelaRow ---")
if ids.get("ku") and ids.get("lv") and ids.get("bpej"):
    parcela_cislo = random.randint(100, 999)
    run_curl("POST", "/parcela_row", {
        "parcelni_cislo": parcela_cislo,
        "cast_parcely": 1,
        "je_stavebni": False,
        "vymera_metru_ctverecnich": "1000.5",
        "ulice": "Hlavni",
        "cislo_popisne": "1",
        "katastralni_uzemi_id": ids["ku"],
        "bpej_id": ids["bpej"],
        "list_vlastnictvi_id": ids["lv"]
    })
    parcely = run_curl("GET", "/parcela_row")
    if isinstance(parcely, list):
        for item in parcely:
            if item["katastralni_uzemi_id"] == ids["ku"] and item["parcelni_cislo"] == parcela_cislo:
                ids["parcela"] = item["id"]
                break
    print(f"Created Parcela ID: {ids['parcela']}")

    # Create another parcela for relations
    run_curl("POST", "/parcela_row", {
        "parcelni_cislo": parcela_cislo + 1,
        "cast_parcely": 1,
        "je_stavebni": False,
        "vymera_metru_ctverecnich": "500",
        "ulice": "Vedlejsi",
        "cislo_popisne": "2",
        "katastralni_uzemi_id": ids["ku"],
        "bpej_id": ids["bpej"],
        "list_vlastnictvi_id": ids["lv"]
    })
    parcely = run_curl("GET", "/parcela_row")
    if isinstance(parcely, list):
        for item in parcely:
            if item["katastralni_uzemi_id"] == ids["ku"] and item["parcelni_cislo"] == parcela_cislo + 1:
                ids["parcela2"] = item["id"]
                break
    print(f"Created Parcela2 ID: {ids['parcela2']}")


# 13. Rizeni
print("\n--- Testing Rizeni ---")
if ids.get("typ_rizeni"):
    rizeni_cislo = random.randint(2025000, 2025999)
    run_curl("POST", "/rizeni", {
        "rok": 2025,
        "cislo_rizeni": rizeni_cislo,
        "typ_rizeni_id": ids["typ_rizeni"],
        "predmet": "Vklad",
        "poznamka": None
    })
    rizenis = run_curl("GET", "/rizeni")
    if isinstance(rizenis, list):
        for item in rizenis:
            if item["cislo_rizeni"] == rizeni_cislo:
                ids["rizeni"] = item["id"]
                break
    print(f"Created Rizeni ID: {ids['rizeni']}")

# 14. Vlastnictvi
print("\n--- Testing Vlastnictvi ---")
if ids.get("parcela") and ids.get("majitel"):
    run_curl("POST", "/vlastnictvi", {
        "parcela_id": ids["parcela"],
        "majitel_id": ids["majitel"],
        "podil_setin": 100
    })
    # Verify
    vlastnictvis = run_curl("GET", "/vlastnictvi")
    found = False
    if isinstance(vlastnictvis, list):
        for item in vlastnictvis:
            if item["parcela_id"] == ids["parcela"] and item["majitel_id"] == ids["majitel"]:
                found = True
                break
    print(f"Vlastnictvi created: {found}")

# 15. BremenoParcelaParcela
print("\n--- Testing BremenoParcelaParcela ---")
if ids.get("parcela") and ids.get("parcela2"):
    run_curl("POST", "/bremeno_parcela_parcela", {
        "parcela_id": ids["parcela"],
        "parcela_povinna_id": ids["parcela2"],
        "popis": "Cesta",
        "datum_zrizeni": "2025-01-01",
        "datum_pravnich_ucinku": "2025-01-01"
    })

# 16. BremenoParcelaMajitel
print("\n--- Testing BremenoParcelaMajitel ---")
if ids.get("parcela") and ids.get("majitel"):
    run_curl("POST", "/bremeno_parcela_majitel", {
        "parcela_id": ids["parcela"],
        "majitel_povinny_id": ids["majitel"],
        "popis": "Uzivani",
        "datum_zrizeni": "2025-01-01",
        "datum_pravnich_ucinku": "2025-01-01"
    })

# 17. RizeniOperaceRow
print("\n--- Testing RizeniOperaceRow ---")
if ids.get("rizeni") and ids.get("typ_operace"):
    run_curl("POST", "/rizeni_operace", {
        "rizeni_id": ids["rizeni"],
        "typ_operace_id": ids["typ_operace"],
        "datum": "2025-01-01"
    })

# 18. Plomba
print("\n--- Testing Plomba ---")
if ids.get("rizeni") and ids.get("parcela"):
    run_curl("POST", "/plomba", {
        "rizeni_id": ids["rizeni"],
        "parcela_id": ids["parcela"]
    })

# 19. Ucast
print("\n--- Testing Ucast ---")
if ids.get("rizeni") and ids.get("ucastnik_rizeni") and ids.get("typ_ucastnika"):
    run_curl("POST", "/ucast", {
        "rizeni_id": ids["rizeni"],
        "ucastnik_rizeni_id": ids["ucastnik_rizeni"],
        "typ_ucastnika_id": ids["typ_ucastnika"]
    })

# --- Special Endpoints ---

print("\n--- Testing /parcela ---")
if ids.get("ku"):
    run_curl("GET", "/parcela", params={
        "katastralni_uzemi": ku_name,
        "parcelni_cislo": parcela_cislo,
        "cast_parcely": 1,
        "je_stavebni": "false"
    })

print("\n--- Testing /lv ---")
if ids.get("ku"):
    run_curl("GET", "/lv", params={
        "katastralni_uzemi": ku_name,
        "cislo_lv": lv_cislo
    })

print("\n--- Testing /spravni_rizeni ---")
if ids.get("rizeni"):
    run_curl("GET", "/spravni_rizeni", params={
        "id": ids["rizeni"]
    })

# --- Cleanup (Delete) ---
# Reverse order of creation roughly

print("\n--- Cleanup ---")

if ids.get("rizeni") and ids.get("ucastnik_rizeni") and ids.get("typ_ucastnika"):
    run_curl("DELETE", "/ucast", params={
        "rizeni_id": ids["rizeni"],
        "ucastnik_rizeni_id": ids["ucastnik_rizeni"],
        "typ_ucastnika_id": ids["typ_ucastnika"]
    })

if ids.get("rizeni") and ids.get("parcela"):
    run_curl("DELETE", "/plomba", params={
        "rizeni_id": ids["rizeni"],
        "parcela_id": ids["parcela"]
    })

if ids.get("rizeni") and ids.get("typ_operace"):
    run_curl("DELETE", "/rizeni_operace", params={
        "rizeni_id": ids["rizeni"],
        "typ_operace_id": ids["typ_operace"]
    })

if ids.get("parcela") and ids.get("majitel"):
    run_curl("DELETE", "/bremeno_parcela_majitel", params={
        "parcela_id": ids["parcela"],
        "majitel_povinny_id": ids["majitel"]
    })

if ids.get("parcela") and ids.get("parcela2"):
    run_curl("DELETE", "/bremeno_parcela_parcela", params={
        "parcela_id": ids["parcela"],
        "parcela_povinna_id": ids["parcela2"]
    })

if ids.get("parcela") and ids.get("majitel"):
    run_curl("DELETE", "/vlastnictvi", params={
        "parcela_id": ids["parcela"],
        "majitel_id": ids["majitel"]
    })

if ids.get("rizeni"):
    run_curl("DELETE", "/rizeni", params={"id": ids["rizeni"]})
if ids.get("parcela"):
    run_curl("DELETE", "/parcela_row", params={"id": ids["parcela"]})
if ids.get("parcela2"):
    run_curl("DELETE", "/parcela_row", params={"id": ids["parcela2"]})
if ids.get("lv"):
    run_curl("DELETE", "/list_vlastnictvi", params={"id": ids["lv"]})
if ids.get("majitel"):
    run_curl("DELETE", "/majitel", params={"id": ids["majitel"]})
if ids.get("ucastnik_rizeni"):
    run_curl("DELETE", "/ucastnik_rizeni", params={"id": ids["ucastnik_rizeni"]})
if ids.get("typ_ucastnika"):
    run_curl("DELETE", "/typ_ucastnika", params={"id": ids["typ_ucastnika"]})
if ids.get("typ_operace"):
    run_curl("DELETE", "/typ_operace", params={"id": ids["typ_operace"]})
if ids.get("typ_rizeni"):
    run_curl("DELETE", "/typ_rizeni", params={"id": ids["typ_rizeni"]})
if ids.get("bpej"):
    run_curl("DELETE", "/bpej", params={"id": ids["bpej"]})
if ids.get("ku"):
    run_curl("DELETE", "/katastralni_uzemi", params={"id": ids["ku"]})
if ids.get("obec"):
    run_curl("DELETE", "/obec", params={"id": ids["obec"]})
if ids.get("okres"):
    run_curl("DELETE", "/okres", params={"id": ids["okres"]})
if ids.get("kraj"):
    run_curl("DELETE", "/kraj", params={"id": ids["kraj"]})

print("\nDone.")

