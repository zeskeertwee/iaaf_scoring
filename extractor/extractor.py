import os.path

from tabula import read_pdf
from os.path import exists
import pickle, datetime, csv, math


def extract_frame(file: str, ext: str, pages: (int, int)):
    pickle_fname = file.replace(".pdf", ext + ".pkl")

    if exists(pickle_fname):
        print("Reading dataframe from " + pickle_fname)
        file = open(pickle_fname, "rb")
        df = pickle.load(file)
        file.close()
        return df

    pagerange = str(pages[0]) + "-" + str(pages[1])
    print("Extracting dataframe from " + file + " (Pages " + pagerange + ") - SEN" + ext)
    df = read_pdf(file, pages=pagerange)
    print("Saving dataframe to " + pickle_fname)
    file = open(pickle_fname, "wb")
    pickle.dump(df, file)
    file.close()
    return df

def parse_timetamp(ts: str):
    formats = [
        "%H:%M:%S",     # HH:MM:SS
        "%H:%M:%S.%f",  # HH:MM:SS.mm
        "%M:%S",        # MM:SS
        "%M:%S.%f",     # MM:SS.mm
        "%S.%f",        # SS.mm
    ]
    for f in formats:
        try:
            return datetime.datetime.strptime(ts, f).time()
        except ValueError:
            pass
    return None


def generate_tables(df):
    tables = {}

    print("Analyzing data")
    for frame in df:
        headers = list(frame)
        print("Found headers " + str(headers))

        unnamed = False
        for h in headers:
            if "Unnamed" in h:
                unnamed = True
                break

        if unnamed:
            print("Skipping unnamed header")
            continue

        for header in headers:
            if header == "Points":
                continue

            for idx, perf in enumerate(frame[header]):
                if perf == "-":
                    continue

                if type(perf) is float:
                    if math.isnan(perf):
                        continue

                nperf = parse_timetamp(str(perf))
                sperf = 0.0
                if nperf:
                    sperf = float(nperf.second) + float(nperf.microsecond) / pow(10, 6) + float(
                        nperf.minute) * 60 + float(nperf.hour) * (60 * 60)
                else:
                    try:
                        # probably heptalon/pentalon point count
                        sperf = int(perf)
                    except:
                        print("Failed to parse " + str(perf))
                        raise Exception("Parsing failure")

                sperf = round(sperf, 3)
                points = int(frame["Points"][idx])

                if not header in tables.keys():
                    tables[header] = []

                tables[header].append((sperf, points))

    sorted_tables = {}
    for key in list(tables):
        print("Sorted table " + key)
        sorted_tables[key] = sorted(tables[key], key=lambda perf: -perf[1])

    return sorted_tables


def save_table(table, file: str):
    f = open(file, "w")
    writer = csv.writer(f)

    writer.writerow(['performance', 'points'])
    writer.writerows(table)

    f.close()
    print("Wrote " + file)


replacements = {
    "Hept.": "Heptathlon",
    "Pent.": "Pentathlon",
    "2000mSC": "2000m SC",
    "3000mSC": "3000m SC",
    "10,000m": "10000m",
    "5km": "5 km",
    "10km": "10 km",
    "15km": "15 km",
    "20km": "20 km",
    "25km": "25 km",
    "30km": "30 km",
    "100km": "100 km",

    "3kmW": "3km W",
    "5kmW": "5km W",
    "10kmW": "10km W",
    "15kmW": "15km W",
    "20kmW": "20km W",
    "30kmW": "30km W",
    "35kmW": "35km W",
    "50kmW": "50km W",

    "10,000mW": "10000mW",
    "15,000mW": "15000mW",
    "20,000mW": "20000mW",
    "30,000mW": "30000mW",
    "35,000mW": "35000mW",
    "50,000mW": "50000mW"
}

def extract_data(file: str, folder: str, export: str, m_pages: (int, int), w_pages: (int, int)):
    if not os.path.exists(folder):
        os.makedirs(folder)

    dfm = extract_frame(file, "-M", m_pages)
    tables_m = generate_tables(dfm)

    for i in list(tables_m):
        dispi = i
        if dispi in replacements:
            dispi = replacements[dispi]

        if "." in dispi:
            raise Exception("Abbreviation not replaced! " + dispi)

        path = folder + export + " - MALE - " + dispi + ".csv"
        save_table(tables_m[i], path)

    dfw = extract_frame(file, "-W", w_pages)
    tables_w = generate_tables(dfw)

    for i in list(tables_w):
        dispi = i
        if dispi in replacements:
            dispi = replacements[dispi]

        if "." in dispi:
            raise Exception("Abbreviation not replaced! " + dispi)

        path = folder + export + " - FEMALE - " + dispi + ".csv"
        save_table(tables_w[i], path)

    #print(str(tables))


folder2022 = "../resources/wa_2022_tables_"
folder2017 = "../resources/iaaf_2017_tables_"

extract_data("IAAF-2017-Indoor.pdf", folder2017 + "indoor/", "Table Indoor 2017", (10, 127), (130, 247))
extract_data("IAAF-2017-Outdoor.pdf", folder2017 + "outdoor/", "Table Outdoor 2017", (10, 187), (190, 367))
extract_data("WA-2022-Indoor.pdf", folder2022 + "indoor/","Table Indoor 2022", (7,154), (157, 304))
extract_data("WA-2022-Outdoor.pdf", folder2022 + "outdoor/", "Table Outdoor 2022", (9, 276), (279, 546))
