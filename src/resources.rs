use std::collections::HashMap;

/// מבנה נתונים המייצג מקור מידע או כלי
#[derive(Debug, Clone)]
pub struct Resource {
    pub name: String,
    pub url: String,
    pub description: String,
    pub category: ResourceCategory,
    pub subcategory: Option<String>,
    pub technologies: Vec<String>,
    pub license: Option<String>,
    pub language: Option<String>,
    pub size: Option<String>,
}

/// קטגוריות של משאבים
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCategory {
    Corpus,                  // קורפוס
    LexicalResource,         // משאב לקסיקלי 
    WordEmbedding,          // וקטורי מילים
    AnnotationTool,         // כלי אנוטציה
    CollaborativeProject,    // פרויקט שיתופי
    Evaluation,             // הערכה
    AcademicLab,            // מעבדה אקדמית
    NonProfit,              // ארגון ללא מטרות רווח
    Industry,               // תעשייה
    Course,                 // קורס
    AudioCorpus,            // קורפוס שמע
    TechnicalCorpus,        // קורפוס טכני
    MaintenanceData,        // נתוני תחזוקה
    TechnicalStandard,      // תקן טכני
    TechnicalOntology,      // אונטולוגיה טכנית
}

/// מורמטים נתמכים של קבצי קלט
#[derive(Debug, Clone, PartialEq)]
pub enum InputFormat {
    Conllu,     // ניתוח מורפולוגי בפורמט CONLLU
    Csv,        // ניתוח מורפולוגי בפורמט CSV
    Jsonl,      // ישויות עם שם בפורמט JSONL
    PlainText,  // טקסט רגיל
    Metadata,   // מטא נתונים של הקורפוס
}

impl Resource {
    pub fn get_format(&self) -> Option<InputFormat> {
        self.url.rsplit_once('.')
            .and_then(|(_, ext)| match ext.to_lowercase().as_str() {
                "conllu" => Some(InputFormat::Conllu),
                "csv" => Some(InputFormat::Csv),
                "jsonl" => Some(InputFormat::Jsonl),
                "txt" => Some(InputFormat::PlainText),
                _ => None
            })
    }
}

/// מאגר המשאבים
pub struct HebrewResources {
    resources: HashMap<String, Resource>,
}

impl HebrewResources {
    pub fn new() -> Self {
        let mut resources = HashMap::new();
        
        // קורפוסים לא מתויגים - כללי
        resources.insert("hedc4".to_string(), Resource {
            name: "HeDC4".to_string(),
            url: "https://huggingface.co/datasets/HeNLP/HeDC4".to_string(),
            description: "קורפוס עברי מנוקה ומסונן מ-Common Crawl".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Unannotated/General".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("Apache License 2.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        resources.insert("jpress".to_string(), Resource {
            name: "JPress".to_string(),
            url: "http://www.jpress.org.il".to_string(),
            description: "אוסף עיתונים יהודיים שפורסמו במדינות, שפות ותקופות שונות".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Unannotated/General".to_string()),
            technologies: vec!["NLP".to_string(), "OCR".to_string()],
            license: Some("Custom Terms of Use".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // קורפוסים לא מתויגים - מתמחים
        resources.insert("sefaria".to_string(), Resource {
            name: "Sefaria".to_string(),
            url: "https://github.com/Sefaria/Sefaria-Export/".to_string(),
            description: "טקסטים יהודיים מובנים ומטא-דאטה עם רישיונות ציבוריים חופשיים".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Unannotated/Specialized".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("Various".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        resources.insert("hebrew_songs".to_string(), Resource {
            name: "Hebrew Songs Lyrics".to_string(),
            url: "https://www.kaggle.com/datasets/guybarash/hebrew-songs-lyrics".to_string(),
            description: "כ-15,000 שירים ישראליים שנאספו מאתר שירונט".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Unannotated/Specialized".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("CC BY-SA 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("15,000 songs".to_string()),
        });

        // קורפוסים רב-לשוניים
        resources.insert("oscar".to_string(), Resource {
            name: "OSCAR".to_string(),
            url: "https://oscar-corpus.com/".to_string(),
            description: "קורפוס רב-לשוני גדול שנוצר מסינון של Common Crawl".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Multilingual".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        resources.insert("cc100".to_string(), Resource {
            name: "CC100".to_string(),
            url: "https://data.statmt.org/cc-100/".to_string(),
            description: "קורפוס המכיל נתונים חד-לשוניים עבור יותר מ-100 שפות".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Multilingual".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("MIT".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        // קורפוסים מתויגים לפי משימה - NER
        resources.insert("nemo".to_string(), Resource {
            name: "NEMO".to_string(),
            url: "https://github.com/OnlpLab/NEMO-Corpus".to_string(),
            description: "אנוטציות של ישויות מוכרות בקורפוס העץ העברי".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/NER".to_string()),
            technologies: vec!["NLP".to_string(), "NER".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // קורפוסי שמע
        resources.insert("ivrit_ai_corpus".to_string(), Resource {
            name: "ivrit.ai Corpus".to_string(),
            url: "https://huggingface.co/ivrit-ai".to_string(),
            description: "מאגר נתוני דיבור בעברית הכולל כ-15,000 שעות של הקלטות מתומללות אוטומטית".to_string(),
            category: ResourceCategory::AudioCorpus,
            subcategory: None,
            technologies: vec!["Speech".to_string(), "ASR".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("15,000 hours".to_string()),
        });

        // משאבים לקסיקליים
        resources.insert("mila_lexicon".to_string(), Resource {
            name: "The MILA lexicon of Hebrew words".to_string(),
            url: "http://www.mila.cs.technion.ac.il/resources_lexicons_mila.html".to_string(),
            description: "לקסיקון המכיל כ-25,000 ערכים לשימוש במנתחים מורפולוגיים".to_string(),
            category: ResourceCategory::LexicalResource,
            subcategory: Some("Monolingual".to_string()),
            technologies: vec!["NLP".to_string(), "Morphology".to_string()],
            license: Some("GPLv3".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("25,000 entries".to_string()),
        });

        // קרפוסים רומתויגים - אלות ותשובות
        resources.insert("heq".to_string(), Resource {
            name: "HeQ".to_string(),
            url: "https://github.com/NNLP-IL/Hebrew-Question-Answering-Dataset".to_string(),
            description: "מאגר נתונים של 30,147 שאלות בעברית מודרנית עם תשובות מתוך הטקסט".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/QA".to_string()),
            technologies: vec!["NLP".to_string(), "QA".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("30,147 questions".to_string()),
        });

        resources.insert("parashoot".to_string(), Resource {
            name: "ParaShoot".to_string(),
            url: "https://github.com/omrikeren/ParaShoot".to_string(),
            description: "מאגר נתונים של שאלות ותשובות בסגנון SQuAD, מבוסס על ערכים מויקיפדיה".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/QA".to_string()),
            technologies: vec!["NLP".to_string(), "QA".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: Some("3,000 QA pairs".to_string()),
        });

        // קורפוסים מתויגים - ניתוח רגשות
        resources.insert("hebrew_sentiment".to_string(), Resource {
            name: "Hebrew-Sentiment-Data".to_string(),
            url: "https://github.com/OnlpLab/Hebrew-Sentiment-Data".to_string(),
            description: "מאגר נתונים לניתוח רגשות בעברית, מבוסס על 12,000 תגובות מרשתות חברתיות".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/Sentiment".to_string()),
            technologies: vec!["NLP".to_string(), "Sentiment Analysis".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: Some("12,000 comments".to_string()),
        });

        resources.insert("emotion_ugc".to_string(), Resource {
            name: "Emotion User Generated Content".to_string(),
            url: "https://github.com/avichaychriqui/HeBERT".to_string(),
            description: "מאגר תגובות מאתרי חדשות ישראליים עם תיוג רגשות ורגש כללי".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/Emotion".to_string()),
            technologies: vec!["NLP".to_string(), "Emotion Detection".to_string()],
            license: Some("MIT".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("350K sentences".to_string()),
        });

        // קורפוסים מתויגים - סיווג נושאים
        resources.insert("knesset_topics".to_string(), Resource {
            name: "Knesset Topic Classification".to_string(),
            url: "https://github.com/NitzanBarzilay/KnessetTopicClassification/".to_string(),
            description: "כ-2,700 ציטוטים מישיבות הכנסת מסווגים לשמונה נושאים".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/Topic".to_string()),
            technologies: vec!["NLP".to_string(), "Topic Classification".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: Some("2,700 quotes".to_string()),
        });

        // קורפוסים מתויגים - הסקה טקסטואלית
        resources.insert("hebnli".to_string(), Resource {
            name: "HebNLI".to_string(),
            url: "https://github.com/NNLP-IL/HebNLI".to_string(),
            description: "מאגר נתונים להסקה טקסטואלית בעברית, מבוסס על תרגום של MultiNLI".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/NLI".to_string()),
            technologies: vec!["NLP".to_string(), "NLI".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // קורפוסים מתויגים - סיכום טקסט
        resources.insert("hesum".to_string(), Resource {
            name: "HeSum".to_string(),
            url: "https://github.com/OnlpLab/HeSum".to_string(),
            description: "מאגר נתונים לסיכום אבסטרקטיבי בעברית מודרנית, מכיל 10,000 מאמרים עם סיכומים".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Annotated/Summarization".to_string()),
            technologies: vec!["NLP".to_string(), "Text Summarization".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: Some("10,000 articles".to_string()),
        });

        // וקטורי מילים
        resources.insert("fasttext_hebrew".to_string(), Resource {
            name: "fastText pre-trained word vectors".to_string(),
            url: "https://github.com/facebookresearch/fastText/blob/master/docs/pretrained-vectors.md".to_string(),
            description: "וקטורי מילים שאומנו על ויקיפדיה העברית באמצעות fastText".to_string(),
            category: ResourceCategory::WordEmbedding,
            subcategory: None,
            technologies: vec!["NLP".to_string(), "Word Embeddings".to_string(), "fastText".to_string()],
            license: Some("CC-BY-SA 3.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        resources.insert("hebrew_w2v".to_string(), Resource {
            name: "hebrew-word2vec".to_string(),
            url: "https://github.com/Ronshm/hebrew-word2vec".to_string(),
            description: "וקטורי מילים שאומנו על נתונים מטוויטר, מכיל וקטורים ל-1.4M מילים".to_string(),
            category: ResourceCategory::WordEmbedding,
            subcategory: None,
            technologies: vec!["NLP".to_string(), "Word Embeddings".to_string(), "Word2Vec".to_string()],
            license: Some("Apache License 2.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("1.4M words".to_string()),
        });

        // קרפוסי שמע�וספים
        resources.insert("hebrew_medical_audio".to_string(), Resource {
            name: "Hebrew Medical Audio Dataset - Verbit".to_string(),
            url: "https://huggingface.co/datasets/verbit/hebrew_medical_audio".to_string(),
            description: "מאגר הקלטות של סיכומים קליניים בעברית מ-41 דוברים שונים".to_string(),
            category: ResourceCategory::AudioCorpus,
            subcategory: None,
            technologies: vec!["Speech".to_string(), "Medical".to_string()],
            license: Some("CC BY-NC 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("1000+ recordings".to_string()),
        });

        resources.insert("hebdb".to_string(), Resource {
            name: "HebDB".to_string(),
            url: "https://pages.cs.huji.ac.il/adiyoss-lab/HebDB/".to_string(),
            description: "מאגר נתוני דיבור בעברית עם כ-2500 שעות של הקלטות טבעיות וספונטניות".to_string(),
            category: ResourceCategory::AudioCorpus,
            subcategory: None,
            technologies: vec!["Speech".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: Some("2500 hours".to_string()),
        });

        // לקסיקונים דו-לשוניים
        resources.insert("hebrew_wordnet".to_string(), Resource {
            name: "Hebrew WordNet".to_string(),
            url: "http://www.mila.cs.technion.ac.il/resources_lexicons_wordnet.html".to_string(),
            description: "וורדנט עברי המיושר עם WordNet באנגלית, איטלקית וספרדית".to_string(),
            category: ResourceCategory::LexicalResource,
            subcategory: Some("Multilingual".to_string()),
            technologies: vec!["NLP".to_string(), "WordNet".to_string()],
            license: Some("GPLv3".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        resources.insert("word2word".to_string(), Resource {
            name: "word2word".to_string(),
            url: "https://github.com/Kyubyong/word2word".to_string(),
            description: "תרגומים מילה-למילה עבור 3,564 זוגות שפות, כולל עברית".to_string(),
            category: ResourceCategory::LexicalResource,
            subcategory: Some("Multilingual".to_string()),
            technologies: vec!["NLP".to_string(), "Translation".to_string()],
            license: Some("Apache License 2.0".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        // רשימות מילים ומילונים
        resources.insert("hebrew_stopwords".to_string(), Resource {
            name: "Hebrew Stop Words based on UD".to_string(),
            url: "https://github.com/NNLP-IL/Stop-Words-Hebrew".to_string(),
            description: "רשימת מילות עצירה בעברית שנוצרה באמצעות Universal Dependencies".to_string(),
            category: ResourceCategory::LexicalResource,
            subcategory: Some("Word Lists".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("CC-BY-SA 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        resources.insert("hebrew_wordlists".to_string(), Resource {
            name: "Hebrew WordLists".to_string(),
            url: "https://github.com/eyaler/hebrew_wordlists".to_string(),
            description: "רשימות מילים שימושיות שחולצו מ-Hspell 1.4".to_string(),
            category: ResourceCategory::LexicalResource,
            subcategory: Some("Word Lists".to_string()),
            technologies: vec!["NLP".to_string()],
            license: Some("AGPL-3.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // מודלי שפה מאומנים מראש
        resources.insert("berel".to_string(), Resource {
            name: "BEREL".to_string(),
            url: "https://www.dropbox.com/sh/us98wjb178itjk1/AACWu62ffHJ0zk19i77_rV06a".to_string(),
            description: "מודל שפה מאומן מראש לעברית רבנית".to_string(),
            category: ResourceCategory::WordEmbedding,
            subcategory: Some("Language Models".to_string()),
            technologies: vec!["NLP".to_string(), "BERT".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // קורפוסים מתמחים - אדריכלות ובנייה
        resources.insert("heb_architecture_corpus".to_string(), Resource {
            name: "Hebrew Architecture Corpus".to_string(),
            url: "https://github.com/bdar-lab/heb_architecture_corpus".to_string(),
            description: "קורפוס טקסטואלי בעברית בנושאי בנייה, תכנון ואדריכלות. כולל מסמכים ממגוון מקורות כמו צווים חקיקתיים, הנחיות רגולטוריות, דוחות מחקר, מחקרים אקדמיים וכתבי עת מקצועיים".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: Some("Unannotated/Specialized".to_string()),
            technologies: vec!["NLP".to_string(), "OCR".to_string(), "Architecture".to_string()],
            license: Some("CC BY 4.0".to_string()),
            language: Some("Hebrew".to_string()),
            size: None,
        });

        // כלי TLP
        resources.insert("nestor".to_string(), Resource {
            name: "Nestor".to_string(),
            url: "https://github.com/usnistgov/nestor".to_string(),
            description: "ממשק משתמש גרפי לתיוג נתוני תחזוקה ועיבוד שפה טכנית".to_string(),
            category: ResourceCategory::AnnotationTool,
            subcategory: Some("TLP".to_string()),
            technologies: vec!["TLP".to_string(), "Maintenance".to_string(), "Annotation".to_string()],
            license: Some("Public Domain".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        resources.insert("maintnet".to_string(), Resource {
            name: "MaintNet".to_string(),
            url: "https://github.com/maintnet/maintnet".to_string(),
            description: "ספריה שיתופית של כלים ונתונים לעיבוד שפה טכנית בתחום התחזוקה".to_string(),
            category: ResourceCategory::MaintenanceData,
            subcategory: Some("TLP".to_string()),
            technologies: vec!["TLP".to_string(), "Maintenance".to_string(), "NLP".to_string()],
            license: Some("MIT".to_string()),
            language: Some("Multilingual".to_string()),
            size: None,
        });

        // קורפוסים טכניים
        resources.insert("excavator_maintenance".to_string(), Resource {
            name: "Excavator Maintenance Dataset".to_string(),
            url: "https://github.com/usnistgov/excavator".to_string(),
            description: "מאגר נתוני תחזוקה של מחפרים, כולל גרסאות מנוקות ולא מנוקות".to_string(),
            category: ResourceCategory::TechnicalCorpus,
            subcategory: Some("Maintenance".to_string()),
            technologies: vec!["TLP".to_string(), "Maintenance".to_string()],
            license: Some("Public Domain".to_string()),
            language: Some("English".to_string()),
            size: None,
        });

        // תקנים טכניים
        resources.insert("iso_15926".to_string(), Resource {
            name: "ISO 15926-4:2019".to_string(),
            url: "https://www.iso.org/standard/74389.html".to_string(),
            description: "תקן לתיעוד מידע על מתקני תהליך".to_string(),
            category: ResourceCategory::TechnicalStandard,
            subcategory: None,
            technologies: vec!["Process Plants".to_string(), "Documentation".to_string()],
            license: Some("ISO".to_string()),
            language: Some("English".to_string()),
            size: None,
        });

        // אונטולוגיות טכניות
        resources.insert("romain".to_string(), Resource {
            name: "ROMAIN".to_string(),
            url: "https://github.com/romain-ontology/romain".to_string(),
            description: "אונטולוגיה לניהול תחזוקה".to_string(),
            category: ResourceCategory::TechnicalOntology,
            subcategory: Some("Maintenance".to_string()),
            technologies: vec!["TLP".to_string(), "Maintenance".to_string(), "Ontology".to_string()],
            license: Some("Apache License 2.0".to_string()),
            language: Some("English".to_string()),
            size: None,
        });

        HebrewResources { resources }
    }

    pub fn get_resource(&self, name: &str) -> Option<&Resource> {
        self.resources.get(name)
    }

    pub fn get_resources_by_category(&self, category: &ResourceCategory) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.category == *category)
            .collect()
    }

    pub fn get_resources_by_subcategory(&self, subcategory: &str) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.subcategory.as_ref().map_or(false, |s| s == subcategory))
            .collect()
    }

    pub fn search_resources(&self, query: &str) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| {
                r.name.to_lowercase().contains(&query.to_lowercase()) ||
                r.description.to_lowercase().contains(&query.to_lowercase()) ||
                r.technologies.iter().any(|t| t.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect()
    }

    pub fn get_resources_by_language(&self, language: &str) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.language.as_ref().map_or(false, |l| l == language))
            .collect()
    }

    pub fn get_resources_by_format(&self, format: &InputFormat) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.get_format().map_or(false, |f| &f == format))
            .collect()
    }

    pub fn process_resource(&self, resource: &Resource, text: &str) -> Result<String, String> {
        match resource.get_format() {
            Some(InputFormat::Conllu) => {
                // עיבוד קובץ CONLLU - ניתוח מורפולוגי
                Ok(format!("מעבד ניתוח מורפולוגי מפורמט CONLLU: {}", text))
            },
            Some(InputFormat::Csv) => {
                // עיבוד קובץ CSV - ניתוח מורפולוגי
                Ok(format!("מעבד ניתוח מורפולוגי מפורמט CSV: {}", text))
            },
            Some(InputFormat::Jsonl) => {
                // עיבוד קובץ JSONL - ישויות עם שם
                Ok(format!("מעבד ישויות עם שם מפורמט JSONL: {}", text))
            },
            Some(InputFormat::PlainText) => {
                // עיבוד טקסט רגיל
                Ok(format!("מעבד טקסט רגיל: {}", text))
            },
            Some(InputFormat::Metadata) => {
                // עיבוד מטא נתונים
                Ok(format!("מעבד מטא נתונים: {}", text))
            },
            None => Err("פורמט לא נתמך".to_string())
        }
    }

    pub fn add_corpus_resource(&mut self, name: &str, path: &str, format: InputFormat) {
        let resource = Resource {
            name: name.to_string(),
            url: path.to_string(),
            description: format!("קורפוס בפורמט {:?}", format),
            category: ResourceCategory::Corpus,
            subcategory: Some(match format {
                InputFormat::Conllu | InputFormat::Csv => "Morphological".to_string(),
                InputFormat::Jsonl => "Named Entities".to_string(),
                InputFormat::PlainText => "Raw Text".to_string(),
                InputFormat::Metadata => "Metadata".to_string(),
            }),
            technologies: vec!["NLP".to_string()],
            license: None,
            language: Some("Hebrew".to_string()),
            size: None,
        };
        self.resources.insert(name.to_string(), resource);
    }

    // הוספת פונקציה חדשה לקבלת משאבים טכניים
    pub fn get_technical_resources(&self) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| {
                matches!(r.category, 
                    ResourceCategory::TechnicalCorpus | 
                    ResourceCategory::MaintenanceData |
                    ResourceCategory::TechnicalStandard |
                    ResourceCategory::TechnicalOntology) ||
                r.technologies.iter().any(|t| t == "TLP")
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_creation() {
        let resources = HebrewResources::new();
        assert!(resources.get_resource("hedc4").is_some());
    }

    #[test]
    fn test_category_filter() {
        let resources = HebrewResources::new();
        let corpora = resources.get_resources_by_category(&ResourceCategory::Corpus);
        assert!(!corpora.is_empty());
    }

    #[test]
    fn test_language_filter() {
        let resources = HebrewResources::new();
        let hebrew_resources = resources.get_resources_by_language("Hebrew");
        assert!(!hebrew_resources.is_empty());
    }

    #[test]
    fn test_search() {
        let resources = HebrewResources::new();
        let nlp_resources = resources.search_resources("nlp");
        assert!(!nlp_resources.is_empty());
    }

    #[test]
    fn test_format_detection() {
        let resource = Resource {
            name: "Test".to_string(),
            url: "test.conllu".to_string(),
            description: "Test resource".to_string(),
            category: ResourceCategory::Corpus,
            subcategory: None,
            technologies: vec![],
            license: None,
            language: None,
            size: None,
        };
        assert_eq!(resource.get_format(), Some(InputFormat::Conllu));
    }

    #[test]
    fn test_format_filter() {
        let mut resources = HebrewResources::new();
        resources.add_corpus_resource("test_conllu", "/path/to/test.conllu", InputFormat::Conllu);
        let conllu_resources = resources.get_resources_by_format(&InputFormat::Conllu);
        assert!(!conllu_resources.is_empty());
    }
} 