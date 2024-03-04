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

    print("Extracting dataframe from " + file)
    df = read_pdf(file, pages=(str(pages[0]) + "-" + str(pages[1])))
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
    "Pent.": "Pentathlon"
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