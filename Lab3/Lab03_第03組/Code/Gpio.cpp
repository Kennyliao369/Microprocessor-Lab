#include "Gpio.h"

#include <QFile>

static const QString gpioRoot = QStringLiteral("/sys/class/gpio");

Gpio::Gpio(int pin, bool out)
    : pin_(pin)
{
    write(gpioRoot + "/export", QByteArray::number(pin));
    testMode_ = !setDirection(out);
}

Gpio::~Gpio()
{
    if (!testMode_)
        write(gpioRoot + "/unexport", QByteArray::number(pin_));
}

bool Gpio::setDirection(bool out)
{
    return write(attrPath("direction"), out ? "out" : "in");
}

bool Gpio::setValue(bool on)
{
    if (testMode_) return true;
    return write(attrPath("value"), on ? "1" : "0");
}

QString Gpio::attrPath(const char *attr) const
{
    return QStringLiteral("%1/gpio%2/%3").arg(gpioRoot).arg(pin_).arg(QLatin1String(attr));
}

bool Gpio::write(const QString &path, const QByteArray &data)
{
    QFile f(path);
    if (!f.open(QIODevice::WriteOnly | QIODevice::Unbuffered))
        return false;
    return f.write(data) == data.size();
}