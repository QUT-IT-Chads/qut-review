import './globals.css';
import Header from '@components/Header';

export default function RootLayout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <html>
            <head />
            <body>
                <Header />
                <main>{children}</main>
            </body>
        </html>
    );
}
