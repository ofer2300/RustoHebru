# RustoHebru - מערכת תרגום טכני דו-לשונית

מערכת תרגום מתקדמת המתמחה בתרגום טכני בין עברית לרוסית, עם דגש על דיוק ואיכות בתחומים הנדסיים וטכניים.

## תכונות עיקריות

- ניתוח מורפולוגי מתקדם לעברית ורוסית
- זיהוי וטיפול במונחים טכניים
- ממשק משתמש מודרני ונוח
- תמיכה בלמידה מתמשכת ושיפור התרגומים
- בקרת איכות מובנית

## דרישות מערכת

- Rust 1.70 ומעלה
- PostgreSQL 13 ומעלה
- Redis 6 ומעלה
- Node.js 18 ומעלה (לפיתוח ווב)
- CUDA 11.8 (לשימוש ב-GPU) או CPU בלבד

## התקנה

### התקנה מקומית

1. שכפל את הריפוזיטורי:
```bash
git clone https://github.com/ofer2300/RustoHebru.git
cd RustoHebru
```

2. התקן את התלויות:
```bash
# התקנת libtorch וספריות נדרשות
chmod +x scripts/install_dependencies.sh
./scripts/install_dependencies.sh

# התקנת חבילות Rust
cargo build
```

3. הגדר משתני סביבה:
```bash
cp .env.example .env
# ערוך את .env עם הערכים המתאימים
```

4. הפעל את האפליקציה:
```bash
cargo run
```

### פרסום באמצעות Docker

1. בנה את תמונת הדוקר:
```bash
docker build -t rusto-hebru .
```

2. הפעל את הקונטיינר:
```bash
docker run -p 8080:8080 \
  -e DATABASE_URL=your_db_url \
  -e REDIS_URL=your_redis_url \
  rusto-hebru
```

### פרסום ב-Vercel

1. התקן את Vercel CLI:
```bash
npm i -g vercel
```

2. התחבר ל-Vercel:
```bash
vercel login
```

3. פרסם את האפליקציה:
```bash
vercel --prod
```

## שימוש באפליקציה

האפליקציה זמינה בכתובת: https://rusto-hebru.vercel.app

### גישה ל-API

נקודות קצה עיקריות:
- `POST /api/translate`: תרגום טקסט
- `GET /api/terms`: קבלת מונחים טכניים
- `POST /api/feedback`: שליחת משוב

דוגמה לשימוש ב-API:
```bash
curl -X POST https://rusto-hebru.vercel.app/api/translate \
  -H "Content-Type: application/json" \
  -d '{"text": "שלום", "source": "he", "target": "ru"}'
```

## ניטור וביצועים

האפליקציה כוללת:
- Prometheus metrics בכתובת `/metrics`
- לוגים מפורטים
- ניטור ביצועים בזמן אמת

## אבטחה

- כל הבקשות מוצפנות ב-HTTPS
- הגנה מפני CSRF
- הגבלת קצב בקשות
- סינון קלט

## רישיון

MIT License - ראה קובץ [LICENSE](LICENSE) למידע נוסף.

## תרומה

אנו מעודדים תרומות! אנא קרא את [CONTRIBUTING.md](CONTRIBUTING.md) למידע נוסף.
