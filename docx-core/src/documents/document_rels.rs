use serde::Serialize;

use super::*;
use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRels {
    pub has_comments: bool,
    pub has_numberings: bool,
    pub images: Vec<(String, String)>,
    pub custom_xml_count: usize,
    pub header_count: usize,
    pub footer_count: usize,
}

impl DocumentRels {
    pub fn new() -> DocumentRels {
        Default::default()
    }

    pub fn add_custom_item(mut self) -> Self {
        self.custom_xml_count += 1;
        self
    }

    pub fn add_image(mut self, id: impl Into<String>, path: impl Into<String>) -> Self {
        self.images.push((id.into(), path.into()));
        self
    }
}

impl BuildXML for DocumentRels {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new();
        b = b
            .declaration(None)
            .open_relationships("http://schemas.openxmlformats.org/package/2006/relationships")
            .relationship(
                "rId1",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
                "styles.xml",
            )
            .relationship(
                "rId2",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
                "fontTable.xml",
            )
            .relationship(
                "rId3",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
                "settings.xml",
            )
            .relationship(
                "rId5",
                "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
                "commentsExtended.xml",
            );

        if self.has_comments {
            b = b.relationship(
                "rId6",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
                "comments.xml",
            )
        }

        if self.has_numberings {
            b = b.relationship(
                "rId7",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
                "numbering.xml",
            )
        }

        for i in 0..self.header_count {
            b = b.relationship(
                &create_header_rid(i + 1),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
                &format!("header{}.xml", i + 1),
            )
        }

        for i in 0..self.footer_count {
            b = b.relationship(
                &create_footer_rid(i + 1),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
                &format!("footer{}.xml", i + 1),
            )
        }

        for i in 0..self.custom_xml_count {
            b = b.relationship(
                &format!("rId{}", i + 8),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
                &format!("../customXml/item{}.xml", i + 1),
            )
        }

        for (id, path) in self.images.iter() {
            b = b.relationship(
                id,
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
                path,
            )
        }

        b.close().build()
    }
}
