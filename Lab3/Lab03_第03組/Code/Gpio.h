#ifndef GPIO_H
#define GPIO_H

#include <QString>

class Gpio
{
public:
    explicit Gpio(int pin, bool out = true);
    ~Gpio();

    bool isTestMode() const { return testMode_; }

    bool setDirection(bool out);
    bool setValue(bool on);

private:
    QString attrPath(const char *attr) const;
    bool write(const QString &path, const QByteArray &data);

    int pin_;
    bool testMode_ = false;
};

#endif // GPIO_H
