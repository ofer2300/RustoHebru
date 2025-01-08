use std::sync::Arc;
use dashmap::DashMap;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::collections::HashMap;
use crate::technical_terms::TechnicalTerm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDictionary {
    terms: HashMap<String, TechnicalTerm>,
    domains: Vec<String>,
    sources: Vec<String>,
}

impl TechnicalDictionary {
    pub fn new() -> Self {
        let mut dict = Self {
            terms: HashMap::new(),
            domains: vec![
                "plumbing".to_string(),
                "fire_safety".to_string(),
                "electrical".to_string(),
                "construction".to_string(),
                "ventilation".to_string(),
            ],
            sources: vec![
                "ГОСТ".to_string(),
                "СНиП".to_string(),
                "תקן ישראלי".to_string(),
                "מפרט טכני".to_string(),
            ],
        };
        
        dict.init_plumbing_terms();
        dict.init_fire_safety_terms();
        dict.init_electrical_terms();
        dict.init_construction_terms();
        dict.init_ventilation_terms();
        
        dict
    }

    fn init_plumbing_terms(&mut self) {
        // מונחי אינסטלציה
        self.add_term(TechnicalTerm {
            term_he: "מערכת אספקת מים".to_string(),
            term_ru: "система водоснабжения".to_string(),
            domain: "plumbing".to_string(),
            context: "מערכות אינסטלציה בבניין".to_string(),
            examples: vec![
                "התקנת מערכת אספקת מים חדשה".to_string(),
                "תחזוקת מערכת אספקת המים".to_string(),
            ],
            synonyms: vec![
                "водопроводная система".to_string(),
                "система подачи воды".to_string(),
            ],
            source: "ГОСТ 32144-2013".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "צנרת ראשית".to_string(),
            term_ru: "магистральный трубопровод".to_string(),
            domain: "plumbing".to_string(),
            context: "מערכת הולכת מים ראשית".to_string(),
            examples: vec![
                "החלפת צנרת ראשית בבניין".to_string(),
                "בדיקת לחץ בצנרת הראשית".to_string(),
            ],
            synonyms: vec![
                "главный трубопровод".to_string(),
                "основная магистраль".to_string(),
            ],
            source: "СНиП 2.04.01-85".to_string(),
            confidence: 0.92,
        });
    }

    fn init_fire_safety_terms(&mut self) {
        // מונחי כיבוי אש
        self.add_term(TechnicalTerm {
            term_he: "מערכת כיבוי אוטומטית".to_string(),
            term_ru: "автоматическая система пожаротушения".to_string(),
            domain: "fire_safety".to_string(),
            context: "מערכות בטיחות אש".to_string(),
            examples: vec![
                "התקנת מערכת כיבוי אוטומטית".to_string(),
                "תחזוקת מערכת הכיבוי".to_string(),
            ],
            synonyms: vec![
                "спринклерная система".to_string(),
                "система автоматического пожаротушения".to_string(),
            ],
            source: "ГОСТ Р 51052-2002".to_string(),
            confidence: 0.98,
        });

        self.add_term(TechnicalTerm {
            term_he: "גלאי עשן".to_string(),
            term_ru: "дымовой извещатель".to_string(),
            domain: "fire_safety".to_string(),
            context: "מערכות גילוי אש".to_string(),
            examples: vec![
                "התקנת גלאי עשן בתקרה".to_string(),
                "בדיקת תקינות גלאי העשן".to_string(),
            ],
            synonyms: vec![
                "датчик дыма".to_string(),
                "дымовой детектор".to_string(),
            ],
            source: "ГОСТ Р 53325-2012".to_string(),
            confidence: 0.96,
        });
    }

    fn init_electrical_terms(&mut self) {
        // מונחי חשמל
        self.add_term(TechnicalTerm {
            term_he: "לוח חשמל ראשי".to_string(),
            term_ru: "главный распределительный щит".to_string(),
            domain: "electrical".to_string(),
            context: "מערכות חשמל".to_string(),
            examples: vec![
                "התקנת לוח חשמל ראשי".to_string(),
                "תחזוקת לוח החשמל".to_string(),
            ],
            synonyms: vec![
                "ГРЩ".to_string(),
                "главный электрощит".to_string(),
            ],
            source: "ГОСТ Р 51321.1-2007".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "מפסק זרם".to_string(),
            term_ru: "автоматический выключатель".to_string(),
            domain: "electrical".to_string(),
            context: "מערכות הגנה חשמלית".to_string(),
            examples: vec![
                "החלפת מפסק זרם".to_string(),
                "כיול מפסק הזרם".to_string(),
            ],
            synonyms: vec![
                "автомат".to_string(),
                "прерыватель".to_string(),
            ],
            source: "ГОСТ Р 50345-2010".to_string(),
            confidence: 0.93,
        });
    }

    fn init_construction_terms(&mut self) {
        // מונחים אחרונים מהתמונה
        self.add_term(TechnicalTerm {
            term_he: "דרישות בטיחות בעבודה".to_string(),
            term_ru: "требования охраны труда".to_string(),
            domain: "construction".to_string(),
            context: "בטיחות בעבודה".to_string(),
            examples: vec![
                "יישום דרישות בטיחות בעבודה".to_string(),
                "בדיקת דרישות בטיחות בעבודה".to_string(),
            ],
            synonyms: vec![
                "требования безопасности труда".to_string(),
                "нормы охраны труда".to_string(),
            ],
            source: "ГОСТ 12.0.004-2015".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות איכות סביבה".to_string(),
            term_ru: "требования охраны окружающей среды".to_string(),
            domain: "construction".to_string(),
            context: "איכות סביבה בבנייה".to_string(),
            examples: vec![
                "עמידה בדרישות איכות סביבה".to_string(),
                "בדיקת דרישות איכות סביבה".to_string(),
            ],
            synonyms: vec![
                "экологические требования".to_string(),
                "природоохранные требования".to_string(),
            ],
            source: "ГОСТ Р 54964-2012".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות נגישות".to_string(),
            term_ru: "требования доступности".to_string(),
            domain: "construction".to_string(),
            context: "נגישות במבנים".to_string(),
            examples: vec![
                "עמידה בדרישות נגישות".to_string(),
                "בדיקת דרישות נגישות".to_string(),
            ],
            synonyms: vec![
                "требования для маломобильных групп".to_string(),
                "нормы доступности".to_string(),
            ],
            source: "СП 59.13330.2016".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות אקוסטיקה".to_string(),
            term_ru: "акустические требования".to_string(),
            domain: "construction".to_string(),
            context: "אקוסטיקה במבנים".to_string(),
            examples: vec![
                "עמידה בדרישות אקוסטיקה".to_string(),
                "בדיקת דרישות אקוסטיקה".to_string(),
            ],
            synonyms: vec![
                "требования звукоизоляции".to_string(),
                "нормы шумоизоляции".to_string(),
            ],
            source: "СП 51.13330.2011".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות תרמיות".to_string(),
            term_ru: "тепловые требования".to_string(),
            domain: "construction".to_string(),
            context: "בידוד תרמי".to_string(),
            examples: vec![
                "עמידה בדרישות תרמיות".to_string(),
                "בדיקת דרישות תרמיות".to_string(),
            ],
            synonyms: vec![
                "теплотехнические требования".to_string(),
                "требования теплоизоляции".to_string(),
            ],
            source: "СП 50.13330.2012".to_string(),
            confidence: 0.94,
        });

        //מונחי בקנות ובדיקות נוספים
        self.add_term(TechnicalTerm {
            term_he: "הנחיות מקצועיות".to_string(),
            term_ru: "профессиональные инструкции".to_string(),
            domain: "construction".to_string(),
            context: "הנחיות מקצועיות לביצוע".to_string(),
            examples: vec![
                "כתיבת הנחיות מקצועיות".to_string(),
                "יישום הנחיות מקצועיות".to_string(),
            ],
            synonyms: vec![
                "профессиональные указания".to_string(),
                "специальные инструкции".to_string(),
            ],
            source: "СНиП 3.01.01-85".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקת תקינות".to_string(),
            term_ru: "проверка исправности".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות טכניות".to_string(),
            examples: vec![
                "ביצוע בדיקת תקינות".to_string(),
                "אישור בדיקת תקינות".to_string(),
            ],
            synonyms: vec![
                "контроль исправности".to_string(),
                "проверка работоспособности".to_string(),
            ],
            source: "ГОСТ 31937-2011".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "הוראות התקנה".to_string(),
            term_ru: "инструкции по монтажу".to_string(),
            domain: "construction".to_string(),
            context: "התקנת מערכות".to_string(),
            examples: vec![
                "מתן הוראות התקנה".to_string(),
                "ביצוע לפי הוראות התקנה".to_string(),
            ],
            synonyms: vec![
                "монтажные инструкции".to_string(),
                "указания по установке".to_string(),
            ],
            source: "ГОСТ 2.601-2013".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות מיוחדות".to_string(),
            term_ru: "специальные требования".to_string(),
            domain: "construction".to_string(),
            context: "דרישות ספציפיות לפרויקט".to_string(),
            examples: vec![
                "הגדרת דרישות מיוחדות".to_string(),
                "יישום דרישות מיוחדות".to_string(),
            ],
            synonyms: vec![
                "особые требования".to_string(),
                "дополнительные требования".to_string(),
            ],
            source: "ГОСТ Р 21.1101-2013".to_string(),
            confidence: 0.93,
        });

        self.add_term(TechnicalTerm {
            term_he: "מסמכי ביצוע".to_string(),
            term_ru: "исполнительная документация".to_string(),
            domain: "construction".to_string(),
            context: "תיעוד ביצוע עבודות".to_string(),
            examples: vec![
                "הכנת מסמכי ביצוע".to_string(),
                "בדיקת מסמכי ביצוע".to_string(),
            ],
            synonyms: vec![
                "документация по выполнению".to_string(),
                "рабочая документация".to_string(),
            ],
            source: "СНиП 3.01.01-85".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "הנחיות בטיחות".to_string(),
            term_ru: "инструкции по безопасности".to_string(),
            domain: "construction".to_string(),
            context: "בטיחות בעבודה".to_string(),
            examples: vec![
                "כתיבת הנחיות בטיחות".to_string(),
                "יישום הנחיות בטיחות".to_string(),
            ],
            synonyms: vec![
                "правила безопасности".to_string(),
                "указания по безопасности".to_string(),
            ],
            source: "ГОСТ 12.0.004-2015".to_string(),
            confidence: 0.97,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות קבלה".to_string(),
            term_ru: "приемочные испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות לקבלת המבנה".to_string(),
            examples: vec![
                "ביצוע בדיקות קבלה".to_string(),
                "אישור בדיקות קבלה".to_string(),
            ],
            synonyms: vec![
                "приемочный контроль".to_string(),
                "испытания при приемке".to_string(),
            ],
            source: "ГОСТ 31814-2012".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות תפקוד".to_string(),
            term_ru: "функциональные требования".to_string(),
            domain: "construction".to_string(),
            context: "דרישות ביצועים".to_string(),
            examples: vec![
                "הגדרת דרישות תפקוד".to_string(),
                "בדיקת דרישות תפקוד".to_string(),
            ],
            synonyms: vec![
                "требования к функционированию".to_string(),
                "эксплуатационные требования".to_string(),
            ],
            source: "ГОСТ Р 53195.1-2008".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "הנחיות תחזוקה".to_string(),
            term_ru: "инструкции по обслуживанию".to_string(),
            domain: "construction".to_string(),
            context: "תחזוקת מבנים ומערכות".to_string(),
            examples: vec![
                "כתיבת הנחיות תחזוקה".to_string(),
                "יישום הנחיות תחזוקה".to_string(),
            ],
            synonyms: vec![
                "указания по техобслуживанию".to_string(),
                "правила обслуживания".to_string(),
            ],
            source: "ГОСТ 2.601-2013".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות איכות".to_string(),
            term_ru: "требования к качеству".to_string(),
            domain: "construction".to_string(),
            context: "בקרת איכות".to_string(),
            examples: vec![
                "הגדרת דרישות איכות".to_string(),
                "עמידה בדרישות איכות".to_string(),
            ],
            synonyms: vec![
                "стандарты качества".to_string(),
                "критерии качества".to_string(),
            ],
            source: "ГОСТ Р ИСО 9001-2015".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקת שוומעבדה".to_string(),
            term_ru: "лабораторные испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות חומרים ומוצרים".to_string(),
            examples: vec![
                "ביצוע בדיקות מעבדה לחומרי בנייה".to_string(),
                "תוצאות בדיקות מעבדה".to_string(),
            ],
            synonyms: vec![
                "лабораторный контроль".to_string(),
                "лабораторные анализы".to_string(),
            ],
            source: "ГОСТ ISO/IEC 17025-2019".to_string(),
            confidence: 0.97,
        });

        self.add_term(TechnicalTerm {
            term_he: "תקן מחייב".to_string(),
            term_ru: "обязательный стандарт".to_string(),
            domain: "construction".to_string(),
            context: "תקינה בבנייה".to_string(),
            examples: vec![
                "עמידה בתקן מחייב".to_string(),
                "יישום דרישות התקן המחייב".to_string(),
            ],
            synonyms: vec![
                "обязательные нормы".to_string(),
                "обязательные требования".to_string(),
            ],
            source: "ГОСТ Р 1.0-2012".to_string(),
            confidence: 0.98,
        });

        self.add_term(TechnicalTerm {
            term_he: "הנחיות תפעול".to_string(),
            term_ru: "инструкции по эксплуатации".to_string(),
            domain: "construction".to_string(),
            context: "תפעול מערכות בניין".to_string(),
            examples: vec![
                "כתיבת הנחיות תפעול".to_string(),
                "יישום הנחיות תפעול".to_string(),
            ],
            synonyms: vec![
                "эксплуатационные инструкции".to_string(),
                "руководство по эксплуатации".to_string(),
            ],
            source: "ГОСТ 2.601-2013".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות סביבתיות".to_string(),
            term_ru: "экологические требования".to_string(),
            domain: "construction".to_string(),
            context: "איכות הסביבה בבנייה".to_string(),
            examples: vec![
                "עמידה בדרישות סביבתיות".to_string(),
                "יישום דרישות סביבתיות".to_string(),
            ],
            synonyms: vec![
                "требования по охране окружающей среды".to_string(),
                "природоохранные требования".to_string(),
            ],
            source: "ГОСТ Р 54964-2012".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות שדה".to_string(),
            term_ru: "полевые испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות באתר הבנייה".to_string(),
            examples: vec![
                "ביצוע בדיקות שדה".to_string(),
                "תיעוד בדיקות שדה".to_string(),
            ],
            synonyms: vec![
                "натурные испытания".to_string(),
                "испытания на месте".to_string(),
            ],
            source: "ГОСТ 31937-2011".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "הנחיות פיקוח".to_string(),
            term_ru: "инструкции по надзору".to_string(),
            domain: "construction".to_string(),
            context: "פיקוח על הבנייה".to_string(),
            examples: vec![
                "כתיבת הנחיות פיקוח".to_string(),
                "יישום הנחיות פיקוח".to_string(),
            ],
            synonyms: vec![
                "указания по надзору".to_string(),
                "правила надзора".to_string(),
            ],
            source: "СНиП 12-01-2004".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות אקוסטיות".to_string(),
            term_ru: "акустические требования".to_string(),
            domain: "construction".to_string(),
            context: "בידוד אקוסטי".to_string(),
            examples: vec![
                "עמידה בדרישות אקוסטיות".to_string(),
                "בדיקת דרישות אקוסטיות".to_string(),
            ],
            synonyms: vec![
                "требования по шумоизоляции".to_string(),
                "нормы звукоизоляции".to_string(),
            ],
            source: "СП 51.13330.2011".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "מפרט ביצוע".to_string(),
            term_ru: "технологический регламент".to_string(),
            domain: "construction".to_string(),
            context: "הנחיות לביצוע עבודות".to_string(),
            examples: vec![
                "הכנת מפרט ביצוע".to_string(),
                "עבודה לפי מפרט ביצוע".to_string(),
            ],
            synonyms: vec![
                "технологическая инструкция".to_string(),
                "порядок выполнения работ".to_string(),
            ],
            source: "ГОСТ 3.1001-2011".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות הרסניות".to_string(),
            term_ru: "разрушающие испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות חוזק חומרים".to_string(),
            examples: vec![
                "בצוע בדיקות הרסניות".to_string(),
                "תוצאות בדיקות הרסניות".to_string(),
            ],
            synonyms: vec![
                "разрушающий контроль".to_string(),
                "испытания на разрушение".to_string(),
            ],
            source: "ГОСТ 16504-81".to_string(),
            confidence: 0.97,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות לא הרסניות".to_string(),
            term_ru: "неразрушающие испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות ללא פגיעה בחומר".to_string(),
            examples: vec![
                "ביצוע בדיקות לא הרסניות".to_string(),
                "שיטות בדיקה לא הרסניות".to_string(),
            ],
            synonyms: vec![
                "неразрушающий контроль".to_string(),
                "НК".to_string(),
            ],
            source: "ГОСТ Р 56542-2015".to_string(),
            confidence: 0.98,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות מפנון".to_string(),
            term_ru: "требования к проектированию".to_string(),
            domain: "construction".to_string(),
            context: "תכנון מבנים ומערכות".to_string(),
            examples: vec![
                "הגדרת דרישות תכנון".to_string(),
                "עמידה בדרישות התכנון".to_string(),
            ],
            synonyms: vec![
                "проектные требования".to_string(),
                "требования проектирования".to_string(),
            ],
            source: "СНиП 11-01-95".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "מפרט טכני מיוחד".to_string(),
            term_ru: "специальные технические условия".to_string(),
            domain: "construction".to_string(),
            context: "דרישות טכניות מיוחדות".to_string(),
            examples: vec![
                "הכנת מפרט טכני מיוחד".to_string(),
                "אישור מפרט טכני מיוחד".to_string(),
            ],
            synonyms: vec![
                "СТУ".to_string(),
                "особые технические требования".to_string(),
            ],
            source: "ГОСТ Р 21.1101-2013".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "תיעוד ביצוע".to_string(),
            term_ru: "исполнительная документация".to_string(),
            domain: "construction".to_string(),
            context: "תיעוד עבודות בנייה".to_string(),
            examples: vec![
                "הכנת תיעוד ביצוע".to_string(),
                "בדיקת תיעוד ביצוע".to_string(),
            ],
            synonyms: vec![
                "документация по выполнению".to_string(),
                "отчетная документация".to_string(),
            ],
            source: "СНиП 3.01.01-85".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות מוקדמות".to_string(),
            term_ru: "предварительные испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות לפני ביצוע".to_string(),
            examples: vec![
                "ביצוע בדיקות מוקדמות".to_string(),
                "תוצאות בדיקות מוקדמות".to_string(),
            ],
            synonyms: vec![
                "предварительный контроль".to_string(),
                "первичные испытания".to_string(),
            ],
            source: "ГОСТ 31937-2011".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "בדיקות סופיות".to_string(),
            term_ru: "окончательные испытания".to_string(),
            domain: "construction".to_string(),
            context: "בדיקות בסיום העבודה".to_string(),
            examples: vec![
                "ביצוע בדיקות סופיות".to_string(),
                "אישור בדיקות סופיות".to_string(),
            ],
            synonyms: vec![
                "финальные испытания".to_string(),
                "заключительный контроль".to_string(),
            ],
            source: "ГОСТ 31814-2012".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות בטיחות אש".to_string(),
            term_ru: "требования пожарной безопасности".to_string(),
            domain: "construction".to_string(),
            context: "בטיחות אש במבנים".to_string(),
            examples: vec![
                "עמידה בדרישות בטיחות אש".to_string(),
                "בדיקת דרישות בטיחות אש".to_string(),
            ],
            synonyms: vec![
                "нормы пожарной безопасности".to_string(),
                "противопожарные требования".to_string(),
            ],
            source: "ГОСТ 12.1.004-91".to_string(),
            confidence: 0.97,
        });

        self.add_term(TechnicalTerm {
            term_he: "תקנות בטיחות".to_string(),
            term_ru: "правила безопасности".to_string(),
            domain: "construction".to_string(),
            context: "בטיחות בעבודה".to_string(),
            examples: vec![
                "יישום תקנות בטיחות".to_string(),
                "עדכון תקנות בטיחות".to_string(),
            ],
            synonyms: vec![
                "нормы безопасности".to_string(),
                "требования безопасности".to_string(),
            ],
            source: "ГОСТ 12.0.004-2015".to_string(),
            confidence: 0.96,
        });

        self.add_term(TechnicalTerm {
            term_he: "הנחיות ביצוע מפורטות".to_string(),
            term_ru: "детальные инструкции по выполнению".to_string(),
            domain: "construction".to_string(),
            context: "הוראות עבודה מפורטות".to_string(),
            examples: vec![
                "הכנת הנחיות ביצוע מפורטות".to_string(),
                "יישום הנחיות ביצוע מפורטות".to_string(),
            ],
            synonyms: vec![
                "подробные указания".to_string(),
                "детальные указания по выполнению".to_string(),
            ],
            source: "СНиП 3.01.01-85".to_string(),
            confidence: 0.94,
        });

        self.add_term(TechnicalTerm {
            term_he: "דרישות טכניות כלליות".to_string(),
            term_ru: "общие технические требования".to_string(),
            domain: "construction".to_string(),
            context: "דרישות טכניות בסיסיות".to_string(),
            examples: vec![
                "הגדרת דרישות טכניות כלליות".to_string(),
                "עמידה בדרישות טכניות כלליות".to_string(),
            ],
            synonyms: vec![
                "ОТТ".to_string(),
                "основные технические требования".to_string(),
            ],
            source: "ГОСТ 15.016-2016".to_string(),
            confidence: 0.95,
        });
    }

    fn init_ventilation_terms(&mut self) {
        // מונחי מיזוג אוויר ואוורור
        self.add_term(TechnicalTerm {
            term_he: "מערכת אוורור".to_string(),
            term_ru: "вентиляционная система".to_string(),
            domain: "ventilation".to_string(),
            context: "מערכות מיזוג אוויר".to_string(),
            examples: vec![
                "התקנת מערכת אוורור".to_string(),
                "תחזוקת מערכת האוורור".to_string(),
            ],
            synonyms: vec![
                "система вентиляции".to_string(),
                "воздушная система".to_string(),
            ],
            source: "ГОСТ 30528-97".to_string(),
            confidence: 0.95,
        });

        self.add_term(TechnicalTerm {
            term_he: "מפוח אוורור".to_string(),
            term_ru: "вентиляционный вентилятор".to_string(),
            domain: "ventilation".to_string(),
            context: "ציוד אוורור".to_string(),
            examples: vec![
                "התקנת מפוח אוורור".to_string(),
                "כיוון מהירות המפוח".to_string(),
            ],
            synonyms: vec![
                "вытяжной вентилятор".to_string(),
                "приточный вентилятор".to_string(),
            ],
            source: "ГОСТ 31350-2007".to_string(),
            confidence: 0.94,
        });
    }

    fn add_term(&mut self, term: TechnicalTerm) {
        self.terms.insert(term.term_he.clone(), term.clone());
        self.terms.insert(term.term_ru.clone(), term);
    }

    pub fn get_term(&self, term: &str) -> Option<&TechnicalTerm> {
        self.terms.get(term)
    }

    pub fn get_terms_by_domain(&self, domain: &str) -> Vec<&TechnicalTerm> {
        self.terms.values()
            .filter(|term| term.domain == domain)
            .collect()
    }

    pub fn get_terms_by_source(&self, source: &str) -> Vec<&TechnicalTerm> {
        self.terms.values()
            .filter(|term| term.source == source)
            .collect()
    }

    pub fn search_terms(&self, query: &str) -> Vec<&TechnicalTerm> {
        self.terms.values()
            .filter(|term| {
                term.term_he.contains(query) ||
                term.term_ru.contains(query) ||
                term.synonyms.iter().any(|s| s.contains(query))
            })
            .collect()
    }

    pub fn get_domains(&self) -> &[String] {
        &self.domains
    }

    pub fn get_sources(&self) -> &[String] {
        &self.sources
    }
}

// Construction and Building Standards
pub const TECHNICAL_TERMS: &[(&str, &str)] = &[
    ("СНиП", "תקנות בנייה"),
    ("СТРОИТЕЛЬНЫЕ НОРМЫ И ПРАВИЛА", "תקנות ונהלי בנייה"),
    ("ОБЩИЕ ПОЛОЖЕНИЯ", "הוראות כלליות"),
    ("Система нормативных документов", "מערכת מסמכים נורמטיביים"),
    ("Общие указания", "הנחיות כלליות"),
    ("Порядок разработки", "סדר הפיתוח"),
    ("согласования", "תיאום"),
    ("утверждение", "אישור"),
    ("Регистрация и издание", "רישום ופרסום"),
    ("Внесение изменений", "הכנסת שינויים"),
    ("дополнений", "תוספות"),
    ("Информация", "מידע"),
    ("Структура", "מבנה"),
    ("Предложения", "הצעות"),
    ("разработке", "פיתוח"),
    ("строительству", "בנייה"),
    ("утверждаемых", "מאושרים"),
    ("включения", "הכללה"),
    ("проект", "פרויקט"),
    ("Плана пересмотра", "תוכנית סקירה"),
    ("действующих", "תקפים"),
    ("новых", "חדשים"),
    ("архитектуре", "אדריכלות"),
    ("Техническое задание", "מפרט טכני"),
    ("технико-экономической", "טכנו-כלכלית"),
    ("эффективности", "יעילות"),
    ("внедрения", "הטמעה"),
    ("Протокол разногласий", "פרוטוקול חילוקי דעות"),
    ("Изложение", "ניסוח"),
    ("Образцы оформления", "דוגמאות עיצוב"),
    ("основных глав", "פרקים עיקריים"),
    ("титульных листов", "דפי כותרת"),
    ("обложки", "כריכה"),
    ("общесоюзного", "כלל-איחודי"),
    ("нормативного документа", "מסמך נורמטיבי"),
    ("инструкции", "הוראות"),
    ("нормы", "תקנים"),
    ("правила", "כללים"),
];

// Construction and Building Standards - Additional Terms
pub const ADDITIONAL_TECHNICAL_TERMS: &[(&str, &str)] = &[
    ("ПРИЛОЖЕНИЕ", "נספח"),
    ("Структура I, II, III и IV частей", "מבנה חלקים I, II, III ו-IV"),
    ("Предложения по разработке", "הצעות לפיתוח"),
    ("для включения в проект", "להכללה בפרויקט"),
    ("Плана пересмотра действующих", "תוכנית לעדכון התקנים הקיימים"),
    ("разработки новых нормативных документов", "פיתוח מסמכים נורמטיביים חדשים"),
    ("по строительству и архитектуре", "לבנייה ואדריכלות"),
    ("форма", "טופס"),
    ("Техническое задание", "מפרט טכני"),
    ("Данные об технико-экономической эффективности", "נתונים על יעילות טכנו-כלכלית"),
    ("внедрения нормативного документа", "יישום המסמך הנורמטיבי"),
    ("Протокол разногласий", "פרוטוקול חילוקי דעות"),
    ("Изложение нормативных документов", "ניסוח מסמכים נורמטיביים"),
    ("Образцы оформления основных глав", "דוגמאות לעיצוב פרקים עיקריים"),
    ("Образцы оформления титульных листов", "דוגמאות לעיצוב דפי כותרת"),
    ("Образец оформления обложки", "דוגמה לעיצוב כריכה"),
    ("общесоюзного нормативного документа", "מסמך נורמטיבי כלל-איחודי"),
    ("утверждаемого Госстроем СССР", "המאושר על ידי ועדת הבנייה הממלכתית של ברה\"מ"),
    ("инструкции, нормы, правила", "הוראות, תקנים וכללים"),
    ("Образец оформления титульного листа", "דוגמה לעיצוב דף שער"),
    ("Оформление 1-й страницы текста", "עיצוב העמוד הראשון של הטקסט"),
    ("Оформление сводки заключений", "עיצוב סיכום המסקנות"),
    ("Книга регистрации утвержденных нормативных документов", "ספר רישום המסמכים הנורמטיביים המאושרים"),
    ("Перечень материалов, подлежащих сдаче в архив", "רשימת החומרים המיועדים להפקדה בארכיון"),
    ("Образец текста штампа", "דוגמה לטקסט החותמת"),
    ("Оформление изменения и дополнения", "עיצוב שינויים ותוספות"),
    ("ГОСУДАРСТВЕННЫЙ КОМИТЕТ СССР ПО ДЕЛАМ СТРОИТЕЛЬСТВА", "הוועדה הממלכתית לענייני בנייה של ברה\"מ"),
    ("ГОССТРОЙ СССР", "ועדת הבנייה של ברה\"מ"),
    ("Издание официальное", "מהדורה רשמית"),
    ("Москва", "מוסקבה"),
    ("СТРОИТЕЛЬНЫЕ НОРМЫ И ПРАВИЛА", "תקנות ונהלי בנייה"),
    ("ОБЩИЕ ПОЛОЖЕНИЯ", "הוראות כלליות"),
    ("Система нормативных документов", "מערכת מסמכים נורמטיביים"),
    ("Утверждены постановлением", "אושר בהחלטה"),
    ("Государственного комитета Совета Министров СССР", "של הוועדה הממלכתית של מועצת השרים של ברה\"מ"),
    ("по делам строительства", "לענייני בנייה"),
    ("МОСКВА СТРОЙИЗДАТ", "מוסקבה הוצאת ספרות בנייה"),
];

// Publishing and Editorial Information
pub const PUBLISHING_TERMS: &[(&str, &str)] = &[
    ("Издание главы", "מהדורת הפרק"),
    ("подготовлено на основе", "הוכן על בסיס"),
    ("разработанной Отделом технического нормирования и стандартизации", "שפותח על ידי מחלקת התקינה הטכנית והסטנדרטיזציה"),
    ("с учетом изменений и дополнений", "בהתחשב בשינויים ובתוספות"),
    ("утвержденных постановлениями", "שאושרו בהחלטות"),
    ("С введением в действие этой главы утрачивают силу", "עם כניסת פרק זה לתוקף, מתבטלים"),
    ("Строительные материалы, изделия, конструкции и оборудование", "חומרי בניין, מוצרים, מבנים וציוד"),
    ("Нормы строительного проектирования", "תקני תכנון בנייה"),
    ("Организация и технология строительного производства", "ארגון וטכנולוגיית ייצור הבנייה"),
    ("Инструкция о порядке разработки и утверждения нормативных документов по строительству", "הוראות לגבי נוהל פיתוח ואישור מסמכים נורמטיביים בבנייה"),
    ("Редакторы — инженеры", "עורכים - מהנדסים"),
    ("Редакция инструктивно-нормативной литературы", "מערכת ספרות הנחיות ותקנים"),
    ("Зав. редакцией", "ראש המערכת"),
    ("Мл. редактор", "עורך משנה"),
    ("Технический редактор", "עורך טכני"),
    ("Корректор", "מגיה"),
    ("Сдано в набор", "נמסר לסידור"),
    ("Подписано в печать", "נחתם לדפוס"),
    ("Формат", "פורמט"),
    ("Бумага типографская", "נייר דפוס"),
    ("Гарнитура", "גופן"),
    ("Печать высокая", "דפוס בלט"),
    ("Усл. печ. л.", "גיליונות דפוס תנאי"),
    ("Уч.-изд. л.", "גיליונות מו\"ל"),
    ("Тираж", "תפוצה"),
    ("Цена", "מחיר"),
    ("Строительные нормы и правила", "תקנות ונהלי בנייה"),
    ("Общие положения", "הוראות כלליות"),
    ("Система нормативных документов", "מערכת מסמכים נורמטיביים"),
    ("Владимирская типография", "בית דפוס ולדימיר"),
    ("при Государственном комитете СССР по делам издательств, полиграфии и книжной торговли", "ליד הוועדה הממלכתית של ברה\"מ לענייני הוצאה לאור, דפוס ומסחר בספרים"),
    ("Инструкт.-нормат.", "הנחיות-תקנים"),
    ("вып.", "מהדורה"),
];

// General Instructions and Requirements
pub const GENERAL_INSTRUCTIONS: &[(&str, &str)] = &[
    ("В настоящей главе Строительных норм и правил", "בפרק זה של תקנות ונהלי הבנייה"),
    ("устанавливается порядок разработки новых", "נקבע סדר פיתוח חדש"),
    ("пересмотра действующих нормативных документов", "סקירת המסמכים הנורמטיביים הקיימים"),
    ("представления этих документов на утверждение", "הגשת מסמכים אלה לאישור"),
    ("согласование, введение в действие и издание", "תיאום, הכנסה לתוקף ופרסום"),
    ("порядок их регистрации, хранения и информации о них", "סדר רישומם, שמירתם והמידע עליהם"),
    ("Правила настоящей главы не распространяются", "כללי פרק זה אינם חלים"),
    ("на стандарты в строительстве", "על תקנים בבנייה"),
    ("издание которых установлено в ГОСТ 1.0-68", "שפרסומם נקבע ב-ГОСТ 1.0-68"),
    ("Государственная система стандартизации", "המערכת הממלכתית לתקינה"),
    ("Основными задачами строительных норм и правил является", "המטרות העיקריות של תקנות ונהלי הבנייה הן"),
    ("установление на основе достижений науки и техники", "קביעה על בסיס הישגי המדע והטכנולוגיה"),
    ("единых требований к проектированию и строительству", "דרישות אחידות לתכנון ובנייה"),
    ("предусматривающих снижение сметной стоимости", "המיועדות להפחתת עלויות התקציב"),
    ("сокращение сроков строительства", "קיצור זמני הבנייה"),
    ("применение наиболее рациональных решений", "יישום פתרונות רציונליים יותר"),
    ("при застройке городов, населенных пунктов", "בבניית ערים ויישובים"),
    ("строительстве предприятий, зданий и сооружений", "בניית מפעלים, מבנים ומתקנים"),
    ("экономное использование материальных ресурсов", "שימוש חסכוני במשאבים חומריים"),
    ("повышение уровня индустриализации", "העלאת רמת התיעוש"),
    ("производительности труда в строительстве", "פריון העבודה בבנייה"),
    ("улучшение условий труда", "שיפור תנאי העבודה"),
    ("охрана окружающей среды", "הגנת הסביבה"),
    ("рациональное использование природных ресурсов", "שימוש רציונלי במשאבי טבע"),
    ("нормы и правила по отдельным вопросам проектирования и строительства", "תקנות וכללים בנושאי תכנון ובנייה ספציפיים"),
    ("нормы проектирования и строительства объектов", "תקני תכנון ובניית מבנים"),
    ("нормы расхода строительных материалов", "תקני צריכת חומרי בנייה"),
    ("нормы технологического проектирования", "תקני תכנון טכנולוגי"),
    ("нормы выработки и расценки на строительные работы", "תקני תפוקה ותעריפים לעבודות בנייה"),
    ("правила устройства электроустановок", "כללי התקנת מתקני חשמל"),
    ("правила о договорах на выполнение проектных и изыскательских работ", "כללים לחוזים לביצוע עבודות תכנון וחקירה"),
    ("для капитального строительства", "לבנייה יסודית"),
    ("инструкции, устанавливающие нормы и правила", "הוראות הקובעות תקנים וכללים"),
    ("проектирования предприятий отдельных отраслей промышленности", "תכנון מפעלים בענפי תעשייה שונים"),
    ("зданий и сооружений различного назначения", "מבנים ומתקנים למטרות שונות"),
    ("конструкций и инженерного оборудования", "מבנים וציוד הנדסי"),
    ("производства отдельных видов строительно-монтажных работ", "ביצוע סוגים שונים של עבודות בנייה והרכבה"),
    ("применения материалов, конструкций и изделий", "שימוש בחומרים, מבנים ומוצרים"),
    ("по организации проектно-изыскательских работ", "לארגון עבודות תכנון וחקירה"),
    ("механизации работ", "מיכון עבודות"),
    ("нормированию труда", "תקינת עבודה"),
    ("разработке проектно-сметной документации", "הכנת מסמכי תכנון ותקציב"),
    ("Ведомственные и республиканские нормативные документы", "מסמכים נורמטיביים משרדיים ורפובליקניים"),
    ("не должны содержать требований", "אינם צריכים לכלול דרישות"),
    ("регламентируемых общесоюзными нормативными документами", "המוסדרות במסמכים נורמטיביים כלל-איחודיים"),
];

// Standards and Requirements
pub const STANDARDS_AND_REQUIREMENTS: &[(&str, &str)] = &[
    ("научно-исследовательские и проектные организации", "ארגוני מחקר ותכנון"),
    ("являющиеся ведущими институтами", "המהווים מכונים מובילים"),
    ("по разработке нормативных документов", "בפיתוח מסמכים נורמטיביים"),
    ("при необходимости разрабатывают", "במידת הצורך מפתחים"),
    ("руководства и рекомендации", "הנחיות והמלצות"),
    ("Руководства должны содержать", "ההנחיות צריכות לכלול"),
    ("методические данные для проектирования и строительства", "נתונים מתודיים לתכנון ובנייה"),
    ("Рекомендации разрабатываются научно-исследовательскими институтами", "ההמלצות מפותחות על ידי מכוני מחקר"),
    ("на основе результатов научных исследований", "על בסיס תוצאות מחקרים מדעיים"),
    ("должны быть направлены на дальнейшее совершенствование", "צריכות להיות מכוונות לשיפור נוסף"),
    ("вопросов проектирования и строительства", "של סוגיות תכנון ובנייה"),
    ("Организации, разрабатывающие и издающие", "ארגונים המפתחים ומפרסמים"),
    ("руководства и рекомендации", "הנחיות והמלצות"),
    ("несут ответственность за правильность", "נושאים באחריות לנכונות"),
    ("приведенных в них данных", "הנתונים המובאים בהם"),
    ("техническую и экономическую обоснованность", "הביסוס הטכני והכלכלי"),
    ("Руководства и рекомендации не являются", "הנחיות והמלצות אינן"),
    ("нормативными документами", "מסמכים נורמטיביים"),
    ("Требования к содержанию нормативных документов", "דרישות לתוכן המסמכים הנורמטיביים"),
    ("по проектированию предприятий, зданий и сооружений", "לתכנון מפעלים, מבנים ומתקנים"),
    ("их оборудованию, строительным конструкциям", "הציוד שלהם, מבנים הנדסיים"),
    ("деталям и изделиям", "פרטים ומוצרים"),
    ("устанавливаемые в нормативных документах", "הנקבעים במסמכים נורמטיביים"),
    ("должны быть взаимно увязаны", "חייבים להיות מתואמים הדדית"),
    ("Нормы технологического проектирования", "תקני תכנון טכנולוגי"),
    ("должны быть увязаны со строительными нормами", "חייבים להיות מתואמים עם תקני הבנייה"),
    ("правилами и нормативами по проектированию", "כללים ותקנים לתכנון"),
    ("предприятий, зданий и сооружений", "מפעלים, מבנים ומתקנים"),
    ("санитарными нормами и другими нормативными документами", "תקני תברואה ומסמכים נורמטיביים אחרים"),
    ("Организации, разрабатывающие проекты", "ארגונים המפתחים פרויקטים"),
    ("нормативных документов, несут ответственность", "של מסמכים נורמטיביים, נושאים באחריות"),
    ("за соответствие требований нормативных документов", "להתאמת דרישות המסמכים הנורמטיביים"),
    ("новейшим достижениям науки и техники", "להישגים העדכניים ביותר של המדע והטכנולוגיה"),
    ("за отражение в нормативных документах требований", "לשיקוף במסמכים הנורמטיביים של דרישות"),
    ("направленных на снижение стоимости", "המכוונות להפחתת עלויות"),
    ("сокращение сроков и повышение качества строительства", "קיצור זמנים ושיפור איכות הבנייה"),
    ("рациональное использование материальных ресурсов", "שימוש רציונלי במשאבים חומריים"),
    ("повышение индустриализации и производительности труда", "העלאת התיעוש ופריון העבודה"),
];

// Document Registration and Numbering
pub const DOCUMENT_REGISTRATION: &[(&str, &str)] = &[
    ("В шифре других общесоюзных норм, правил и инструкций", "בקוד של תקנות, כללים והוראות כלל-איחודיות אחרות"),
    ("применяется сокращенное обозначение «СН»", "משתמשים בקיצור \"СН\""),
    ("порядковый номер по регистрационной книге", "מספר סידורי בספר הרישום"),
    ("а две последние цифры соответствуют году утверждения", "ושתי הספרות האחרונות מתאימות לשנת האישור"),
    ("например: «СНиП II-3I-74»", "לדוגמה: \"СНиП II-3I-74\""),
    ("В шифре ведомственных нормативных документов", "בקוד של מסמכים נורמטיביים משרדיים"),
    ("при их регистрации приводится сокращенное обозначение «ВСН»", "ברישומם מופיע הקיצור \"ВСН\""),
    ("порядковый номер по регистрационной книге", "מספר סידורי בספר הרישום"),
    ("с добавлением к нему последних двух цифр года утверждения", "בתוספת שתי הספרות האחרונות של שנת האישור"),
    ("а также сокращенное название органа, утвердившего нормативный документ", "וכן השם המקוצר של הגוף שאישר את המסמך הנורמטיבי"),
    ("например: ВСН 89-75 Минтрансстрой", "לדוגמה: ВСН 89-75 Минтрансстрой"),
    ("В шифре республиканских нормативных документов", "בקוד של מסמכים נורמטיביים רפובליקניים"),
    ("при их регистрации приводится сокращенное обозначение «РСН»", "ברישומם מופיע הקיצור \"РСН\""),
    ("порядковый номер по регистрационной книге", "מספר סידורי בספר הרישום"),
    ("с добавлением к нему двух последних цифр года утверждения", "בתוספת שתי הספרות האחרונות של שנת האישור"),
    ("и сокращенного названия органа, утвердившего нормативный документ", "והשם המקוצר של הגוף שאישר את המסמך הנורמטיבי"),
    ("например: РСН 68-69 Госстрой Белорусской ССР", "לדוגמה: РСН 68-69 Госстрой של הרפובליקה הבלורוסית"),
    ("Если нормативный документ утверждается взамен действующего", "אם המסמך הנורמטיבי מאושר במקום הקיים"),
    ("с тем же названием, то сохраняется его прежний шифр", "עם אותו שם, נשמר הקוד הקודם שלו"),
    ("с соответствующим изменением цифр года утверждения", "עם שינוי מתאים של ספרות שנת האישור"),
    ("После регистрации подлинное дело утвержденного Госстроем СССР", "לאחר הרישום, התיק המקורי שאושר על ידי Госстрой של ברה\"מ"),
    ("нормативного документа со всеми материалами", "של המסמך הנורמטיבי עם כל החומרים"),
    ("перечисленными в прил. 17", "המפורטים בנספח 17"),
    ("должно сдаваться в архив", "חייב להימסר לארכיון"),
    ("отпечатанный и подписанный в установленном порядке", "מודפס וחתום בסדר הקבוע"),
    ("экземпляр утвержденного нормативного документа", "עותק של המסמך הנורמטיבי המאושר"),
    ("с предисловием, указанным в прил. 16", "עם ההקדמה המצוינת בנספח 16"),
    ("при регистрации должен сдаваться в отдел", "בעת הרישום חייב להימסר למחלקה"),
    ("осуществляющий подготовку к изданию", "המבצעת הכנה לפרסום"),
    ("Копия утвержденного Госстроем СССР", "העתק שאושר על ידי Госстрой של ברה\"מ"),
    ("нормативного документа в двух экземплярах", "של המסמך הנורמטיבי בשני עותקים"),
    ("в том числе первый экземпляр, отдел (управление)", "כולל העותק הראשון, המחלקה (הנהלה)"),
    ("ответственный за подготовку документа к утверждению", "האחראית להכנת המסמך לאישור"),
    ("должен направлять в издательство", "חייבת לשלוח להוצאה לאור"),
];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dictionary() {
        let dict = TechnicalDictionary::new();
        
        // בדיקת תרגום מעברית לרוסית
        let term = dict.get_translation("בנייה ירוקה", "he").unwrap();
        assert_eq!(term.russian, "зеленое строительство");
        
        // בדיקת תרגום מרוסית לעברית
        let term = dict.get_translation("система водоснабжения", "ru").unwrap();
        assert_eq!(term.hebrew, "מערכת אספקת מים");
        
        // בדיקת מונחים לפי תחום
        let terms = dict.get_terms_by_domain("מערכות מים");
        assert!(terms.len() >= 2);
    }
} 