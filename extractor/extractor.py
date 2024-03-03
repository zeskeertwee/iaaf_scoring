from tabula import read_pdf
from os.path import exists
import pandas as pd
import pickle, datetime

def extract_frame(file: str):
    pickle_fname = file.replace(".pdf", ".pkl")

    if exists(pickle_fname):
        print("Reading dataframe from " + pickle_fname)
        file = open(pickle_fname, "rb")
        df = pickle.load(file)
        file.close()
        return df

    print("Extracting dataframe from " + file)
    df = read_pdf(file, pages='all')
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

def extract_data(file: str, export: str):
    df = extract_frame(file)

    tables = {}

    print("Analyzing data")
    for frame in df:
        headers = list(frame)
        print("Found headers " + str(headers))

        for header in headers:
            if header == "Points":
                continue

            for idx, perf in enumerate(frame[header]):
                if perf == "-":
                    continue

                nperf = parse_timetamp(str(perf))
                sperf = 0.0
                if nperf:
                    sperf = float(nperf.second) + float(nperf.microsecond) / pow(10, 6) + float(nperf.minute) * 60 + float(nperf.hour) * (60*60)
                else:
                    # probably heptalon/pentalon point count
                    sperf = int(perf)

                sperf = round(sperf, 3)
                points = int(frame["Points"][idx])

                if not header in tables.keys():
                    tables[header] = []

                tables[header].append((points, sperf))

    #print(str(tables))

extract_data("WA-2022-Indoor.pdf", "WA-2022-Indoor")
extract_data("WA-2022-Outdoor.pdf", "WA-2022-Outdoor")