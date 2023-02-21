#include <iostream>
#include <strstream>
#include <sstream>

#include <kytea/corpus-io.h>
#include <kytea/kytea.h>
#include <kytea/kytea-struct.h>
#include <kytea/string-util.h>

using namespace std;
using namespace kytea;

extern "C" void *new_kytea(const char *model) {
    Kytea *kytea;
    kytea = new Kytea;

    KyteaConfig *config = kytea->getConfig();
    config->setDebug(0);
    config->setOnTraining(false);
    config->setWordBound("\t");
    config->setInputFormat(CORP_FORMAT_RAW);

    kytea->readModel(model);

    return (void *)kytea;
}

extern "C" void delete_kytea(void *void_kytea) {
    Kytea *kytea = (Kytea *)void_kytea;
    delete kytea;
}

extern "C" void *new_istream(char *input) {
    stringstream *buf = new stringstream(input, ios_base::in);
    return (void *)buf;
}

extern "C" void delete_istream(void *void_stream) {
    stringstream *buf = (stringstream *)void_stream;
    delete buf;
}

extern "C" void *new_ostream() {
    strstream *buf = new strstream;
    return (void *)buf;
}

extern "C" void delete_ostream(void *void_stream) {
    strstream *buf = (strstream *)void_stream;
    buf->freeze();
    delete buf;
}

extern "C" void rewind_ostream(void *void_stream) {
    strstream *buf = (strstream *)void_stream;
    buf->seekp(0, ios_base::beg);
}

extern "C" typedef struct {
    const char *ptr;
    int size;
} Str;

extern "C" Str ostream_rust(void *void_stream) {
    strstream *buf = (strstream *)void_stream;
    Str str;
    str.ptr = buf->str();
    str.size = buf->pcount();
    buf->freeze(false);
    return str;
}

void run_kytea_with_io(Kytea *kytea, CorpusIO *in, CorpusIO *out) {
    KyteaConfig *config = kytea->getConfig();

    out->setUnkTag(config->getUnkTag());
    out->setNumTags(config->getNumTags());

    for(int i = 0; i < config->getNumTags(); i++)
        out->setDoTag(i, config->getDoTag(i));

    KyteaSentence* next;
    while((next = in->readSentence()) != 0) {
        if(config->getDoWS())
            kytea->calculateWS(*next);
        if(config->getDoTags())
            for(int i = 0; i < config->getNumTags(); i++)
                if(config->getDoTag(i))
                    kytea->calculateTags(*next, i);
        out->writeSentence(next);
        delete next;
    }
}

extern "C" void run_kytea_str_str(void *void_kytea, void *input, void *output) {
    Kytea *kytea = (Kytea *)void_kytea;

    StringUtil *util = kytea->getStringUtil(); 

    KyteaConfig *config = kytea->getConfig();

    CorpusIO *in, *out;
    stringstream *inbuf = (stringstream *)input;
    in = CorpusIO::createIO(*inbuf, config->getInputFormat(), *config, false, util);
    strstream *outbuf = (strstream *)output;
    out = CorpusIO::createIO(*outbuf, config->getOutputFormat(), *config, true, util);

    run_kytea_with_io(kytea, in, out);

    delete in;
    delete out;
}

extern "C" void run_kytea_file_str(void *void_kytea, const char *input, void *output) {
    Kytea *kytea = (Kytea *)void_kytea;

    StringUtil *util = kytea->getStringUtil(); 

    KyteaConfig *config = kytea->getConfig();

    CorpusIO *in, *out;
    in = CorpusIO::createIO(input, config->getInputFormat(), *config, false, util);
    strstream *outbuf = (strstream *)output;
    out = CorpusIO::createIO(*outbuf, config->getOutputFormat(), *config, true, util);

    run_kytea_with_io(kytea, in, out);

    delete in;
    delete out;
}

extern "C" void run_kytea_str_file(void *void_kytea, void *input, const char *output) {
    Kytea *kytea = (Kytea *)void_kytea;

    StringUtil *util = kytea->getStringUtil(); 

    KyteaConfig *config = kytea->getConfig();

    CorpusIO *in, *out;
    stringstream *inbuf = (stringstream *)input;
    in = CorpusIO::createIO(*inbuf, config->getInputFormat(), *config, false, util);
    out = CorpusIO::createIO(output, config->getOutputFormat(), *config, true, util);

    run_kytea_with_io(kytea, in, out);

    delete in;
    delete out;
}

extern "C" void run_kytea_file_file(void *void_kytea, const char *input, const char *output) {
    Kytea *kytea = (Kytea *)void_kytea;

    StringUtil *util = kytea->getStringUtil(); 

    KyteaConfig *config = kytea->getConfig();

    CorpusIO *in, *out;
    in = CorpusIO::createIO(input, config->getInputFormat(), *config, false, util);
    out = CorpusIO::createIO(output, config->getOutputFormat(), *config, true, util);

    run_kytea_with_io(kytea, in, out);

    delete in;
    delete out;
}
